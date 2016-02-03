use viewport::Viewport;
use opengl_graphics::GlGraphics;
use world::Camera;
use super::{
    CollisionBehavior,
    BodyType,
    //BodySnapshot
};
use util::bounding_box_raycast;
use world::spatial_hashing::Location;
use std::f64::consts::PI;
use rand;
use rand::distributions::{IndependentSample, Range};

pub trait BodyTrait {
    fn id(&self) -> usize;

    fn dead(&self) -> bool;

    fn damage(&self, d: f64);

    fn body_type(&self) -> BodyType;

    fn width2(&self) -> f64;

    fn height2(&self) -> f64;

    fn x(&self) -> f64;

    fn set_x(&self, x: f64);

    fn y(&self) -> f64;

    fn set_y(&self, y: f64);

    fn weight(&self) -> f64;

    fn velocity(&self) -> f64;

    fn set_velocity(&self, v: f64);

    fn angle(&self) -> f64;

    fn set_angle(&self, a: f64);

    fn mask(&self) -> u32;

    fn group(&self) -> u32;

    fn collision_behavior(&self) -> CollisionBehavior;

    fn update(&self, dt: f64);

    fn render(&self, viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics);

    fn render_debug(&self, lines: &mut Vec<[f64;4]>);

    fn on_collision(&self, other: &BodyTrait);

    fn up (&self) -> f64 {
        self.y() + self.height2()
    }
    fn down (&self) -> f64 {
        self.y() - self.height2()
    }
    fn left (&self) -> f64 {
        self.x() - self.width2()
    }
    fn right (&self) -> f64 {
        self.x() + self.width2()
    }

    fn collide(&self, other: &BodyTrait) -> bool {
        let a = self;
        let b = other;
        if (a.group() & b.mask() != 0) && (b.group() & a.mask() != 0) {
            let a_min_x = a.x() - a.width2();
            let a_max_x = a.x() + a.width2();
            let a_min_y = a.y() - a.height2();
            let a_max_y = a.y() + a.height2();
            let b_min_x = b.x() - b.width2();
            let b_max_x = b.x() + b.width2();
            let b_min_y = b.y() - b.height2();
            let b_max_y = b.y() + b.height2();

            if (a_min_x >= b_max_x) || (b_min_x >= a_max_x) || (a_min_y >= b_max_y) || (b_min_y >= a_max_y) {
                false
            } else {
                true
            }
        } else {
            false
        }
    }

    fn resolve_collision(&self, other: &BodyTrait) {
        let a = self;
        let b = other;

        let hori_dir;
        let delta;

        {
            let a_min_x = a.x() - a.width2();
            let a_max_x = a.x() + a.width2();
            let a_min_y = a.y() - a.height2();
            let a_max_y = a.y() + a.height2();
            let b_min_x = b.x() - b.width2();
            let b_max_x = b.x() + b.width2();
            let b_min_y = b.y() - b.height2();
            let b_max_y = b.y() + b.height2();

            let delta_ox = b_max_x - a_min_x;
            let delta_oxp = b_min_x - a_max_x;
            let delta_oy = b_max_y - a_min_y;
            let delta_oyp =  b_min_y - a_max_y;

            let delta_x;
            if delta_ox.abs() < delta_oxp.abs() {
                delta_x = delta_ox;
            } else {
                delta_x = delta_oxp;
            }

            let delta_y;
            if delta_oy.abs() < delta_oyp.abs() {
                delta_y = delta_oy;
            } else {
                delta_y = delta_oyp;
            }

            if delta_x.abs() < delta_y.abs() {
                hori_dir = true;
                delta = delta_x;
            } else {
                hori_dir = false;
                delta = delta_y;
            }
        }

        let mut rate = a.weight()/(a.weight()+b.weight());
        if rate.is_nan() {
            rate = 1.;
        }

        if hori_dir {
            let x = a.x();
            a.set_x(x + (1.-rate)*delta);
        } else {
            let y = a.y();
            a.set_y(y + (1.-rate)*delta);
        }
        match a.collision_behavior() {
            CollisionBehavior::Bounce => {
                let an = if hori_dir {
                    PI - a.angle()
                } else {
                    - a.angle()
                };
                a.set_angle(an);
            },
            CollisionBehavior::Random => {
                let range = Range::new(-PI,PI);
                let mut rng = rand::thread_rng();
                a.set_angle(range.ind_sample(&mut rng));
            },
            CollisionBehavior::Stop => a.set_velocity(0.),
            CollisionBehavior::Persist => (),
        }
    }

    fn location(&self) -> Location {
        Location {
            up: self.y() + self.height2(),
            down: self.y() - self.height2(),
            left: self.x() - self.width2(),
            right: self.x() + self.width2(),
        }
    }

    fn raycast(&self, a: f64, b: f64, c: f64) -> Option<(f64,f64,f64,f64)> {
        bounding_box_raycast(self.x(),self.y(),self.width2()*2.,self.height2()*2.,a,b,c)
    }

    fn in_circle(&self, origin: [f64;2], radius: f64) -> bool {
        let trans = vec![
            [self.left()-origin[0],self.down()-origin[1]],
            [self.left()-origin[0],self.up()-origin[1]],
            [self.right()-origin[0],self.down()-origin[1]],
            [self.right()-origin[0],self.up()-origin[1]]
        ];

        let alpha = {
            let mut index = 0;
            let mut min = trans[0][0].powi(2) + trans[0][1].powi(2);
            for p in 1..4 {
                let d = trans[p][0].powi(2) + trans[p][1].powi(2);
                if d < min {
                    min = d;
                    index = p
                }
            } 

            trans[index][1].atan2(trans[index][0])
        };

        let mut projections = Vec::new();
        for p in trans {
            let proj_x = p[0]*alpha.cos() + p[1]*alpha.sin();
            let proj_y = -p[0]*alpha.sin() + p[1]*alpha.cos();

            projections.push([proj_x,proj_y]);
        }
        let mut min_x = projections[0][0];
        let mut max_x = projections[0][0];
        let mut min_y = projections[0][1];
        let mut max_y = projections[0][1];
        for i in 1..4 {
            let x = projections[i][0];
            let y = projections[i][1];
            if x < min_x {
                min_x = x;
            } else if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            } else if y > max_y {
                max_y = y;
            }
        }

        if (min_x >= radius) || (-radius >= max_x) || (min_y >= radius) || (-radius >= max_y) {
            false
        } else {
            true
        }
    }

    fn in_location(&self,loc: &Location) -> bool {
        if (self.left() >= loc.right) || (loc.left >= self.right()) || (self.down() >= loc.up) || (loc.down >= self.up()) {
            false
        } else {
            true
        }
    }

    //fn delta_snapshot(&mut self) -> Option<BodySnapshot>;

    //fn interpolate(&mut self, from: &BodySnapshot, to: &BodySnapshot, percent: f64);
}

//impl BodyTrait for Rc<BodyTrait> {
//    fn id(&self) -> usize {
//        (&**self).id()
//    }
//
//    fn width2(&self) -> f64 {
//        (&**self).width2()
//    }
//
//    fn height2(&self) -> f64 {
//        (&**self).height2()
//    }
//
//    fn x(&self) -> f64 {
//        (&**self).x()
//    }
//
//    fn set_x(&mut self, x: f64) {
//    }
//
//    fn y(&self) -> f64 {
//        (&**self).y()
//    }
//
//    fn set_y(&mut self, y: f64) {
//    }
//
//    fn weight(&self) -> f64 {
//        (&**self).weight()
//    }
//
//    fn velocity(&self) -> f64 {
//        (&**self).velocity()
//    }
//
//    fn set_velocity(&mut self, v: f64) {
//    }
//
//    fn angle(&self) -> f64 {
//        (&**self).angle()
//    }
//
//    fn set_angle(&mut self, a: f64) {
//    }
//
//    fn mask(&self) -> u32 {
//        (&**self).mask()
//    }
//
//    fn group(&self) -> u32 {
//        (&**self).group()
//    }
//
//    fn collision_behavior(&self) -> CollisionBehavior {
//        (&**self).collision_behavior()
//    }
//
//    fn update(&mut self, dt: f64) {
//        (self as &mut BodyTrait).update(dt);
//    }
//
//    fn render(&self, viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) {
//        (&**self).render(viewport,camera,gl);
//    }
//
//
//    fn on_collision(&mut self, other: &BodyTrait) {
//    }
//
//    //fn delta_snapshot(&mut self) -> Option<BodySnapshot>;
//
//    //fn interpolate(&mut self, from: &BodySnapshot, to: &BodySnapshot, percent: f64);
//}
//
//impl BodyTrait for Rc<RefCell<BodyTrait>> {
//    fn id(&self) -> usize {
//        self.borrow().id()
//    }
//
//    fn width2(&self) -> f64 {
//        self.borrow().width2()
//    }
//
//    fn height2(&self) -> f64 {
//        self.borrow().height2()
//    }
//
//    fn x(&self) -> f64 {
//        self.borrow().x()
//    }
//
//    fn set_x(&mut self, x: f64) {
//        self.borrow_mut().set_x(x);
//    }
//
//    fn y(&self) -> f64 {
//        self.borrow().y()
//    }
//
//    fn set_y(&mut self, y: f64) {
//        self.borrow_mut().set_y(y);
//    }
//
//    fn weight(&self) -> f64 {
//        self.borrow().weight()
//    }
//
//    fn velocity(&self) -> f64 {
//        self.borrow().velocity()
//    }
//
//    fn set_velocity(&mut self, v: f64) {
//        self.borrow_mut().set_velocity(v);
//    }
//
//    fn angle(&self) -> f64 {
//        self.borrow().angle()
//    }
//
//    fn set_angle(&mut self, a: f64) {
//        self.borrow_mut().set_angle(a);
//    }
//
//    fn mask(&self) -> u32 {
//        self.borrow().mask()
//    }
//
//    fn group(&self) -> u32 {
//        self.borrow().group()
//    }
//
//    fn collision_behavior(&self) -> CollisionBehavior {
//        self.borrow().collision_behavior()
//    }
//
//    fn update(&mut self, dt: f64) {
//        self.borrow_mut().update(dt)
//    }
//
//    fn render(&self, viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) {
//        self.borrow().render(viewport,camera,gl);
//    }
//
//
//    fn on_collision(&mut self, other: &BodyTrait) {
//        self.borrow_mut().on_collision(other);
//    }
//
//    //fn delta_snapshot(&mut self) -> Option<BodySnapshot>;
//
//    //fn interpolate(&mut self, from: &BodySnapshot, to: &BodySnapshot, percent: f64);
//}
