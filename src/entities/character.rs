use rand;
use super::group;
use super::Grenade;
use std::rc::Rc;
use rand::distributions::{IndependentSample, Range};
use world::body::{Location, CollisionBehavior, PhysicType, Body, Item};
use world::{World, Entity, EntityCell};
use std::cell::{RefCell, Ref, RefMut};
use std::f64;
use utils::minus_pi_pi;
use frame_manager::{FrameManager, Animation};
use effect_manager::{EffectManager, Position, Line, Effect};

pub const LIFE: f64 = 10000000.;
pub const WIDTH: f64 = 1.;
pub const HEIGHT: f64 = 1.;
pub const WEIGHT: f64 = 1.;
pub const VELOCITY: f64 = 55.;
pub const MASK: u64 = !0;
pub const GROUP: u64 = super::group::CHARACTER;
pub const COLLISION_BEHAVIOR: CollisionBehavior = CollisionBehavior::Persist;
pub const PHYSIC_TYPE: PhysicType = PhysicType::Dynamic;

pub const SWORD_RECOVER: f64 = 0.8;
pub const SWORD_LENGTH: f64 = 5.;
pub const SWORD_DAMAGE: f64 = 5.;

pub const RIFLE_RELOADING_TIME: f64 = 0.1;
pub const SHOTGUN_RELOADING_TIME: f64 = 0.8;
pub const SNIPER_RELOADING_TIME: f64 = 1.5;

pub const RIFLE_LENGTH: f64 = 30.;
pub const SHOTGUN_LENGTH: f64 = 30.;
pub const SNIPER_LENGTH: f64 = 70.;

pub const RIFLE_DAMAGE: f64 = 10.;
pub const SHOTGUN_DAMAGE: f64 = 10.;
pub const SNIPER_DAMAGE: f64 = 100.;

pub const RIFLE_MAX_DELTA_ANGLE: f64 = f64::consts::PI/16.;
pub const SHOTGUN_MAX_DELTA_ANGLE: f64 = f64::consts::PI/6.;
pub const SHOTGUN_SHOOT_NUMBER: u64 = 4;
pub const GRENADE_DISTANCE: f64 = 5.;

pub struct Character {
    body: Body,
    aim: f64,
    gun: Gun,
    sword: Sword,
}

impl Character {
    pub fn new(x: f64, y: f64, angle: f64) -> Character {
        Character {
            body: Body {
                id: 0,
                x: x,
                y: y,
                life: LIFE,
                width: WIDTH,
                height: HEIGHT,
                weight: WEIGHT,
                velocity: 0.,
                angle: angle,
                mask: MASK,
                items: Vec::new(),
                group: GROUP,
                collision_behavior: COLLISION_BEHAVIOR,
                physic_type: PHYSIC_TYPE,
            },
            aim: angle,
            gun: Gun::new(),
            sword: Sword::new(),
        }
    }
}

pub trait CharacterManager {
    fn position(&self) -> (f64,f64);
    fn aim(&self) -> f64;
    fn set_aim(&self, aim: f64);
    fn velocity(&self) -> f64;
    fn set_velocity(&self, v: f64);
    fn angle(&self) -> f64;
    fn set_angle(&self, a: f64);
    fn set_attack_sword(&self);
    fn set_shoot(&self, shoot: bool);
    fn set_launch_grenade(&self, world: &mut World);
}

impl CharacterManager for RefCell<Character> {
    fn position(&self) -> (f64,f64) {
        (self.borrow().body.x,self.borrow().body.y)
    }

    fn aim(&self) -> f64 {
        self.borrow().aim
    }

    fn set_aim(&self, aim: f64) {
        self.borrow_mut().aim = aim;
    }

    fn velocity(&self) -> f64 {
        self.borrow().body.velocity
    }

    fn set_velocity(&self, v: f64) {
        let v = v.min(1.0);
        self.borrow_mut().body.velocity = v*VELOCITY;
    }

    fn angle(&self) -> f64 {
        self.borrow().body.angle
    }

    fn set_angle(&self, a: f64) {
        self.borrow_mut().body.angle = a;
    }

    fn set_attack_sword(&self) {
        self.borrow_mut().sword.do_attack = true;
    }

    fn set_shoot(&self, shoot: bool) {
        self.borrow_mut().gun.shooting = shoot;
    }

    fn set_launch_grenade(&self, world: &mut World) {
        let aim = self.aim();
        let x = self.borrow().body.x + GRENADE_DISTANCE*aim.cos();
        let y = self.borrow().body.y + GRENADE_DISTANCE*aim.sin();
        world.insert(&(Rc::new(RefCell::new(Grenade::new(x,y,aim))) as Rc<EntityCell>));
    }
}

impl EntityCell for RefCell<Character> {
    fn borrow(&self) -> Ref<Entity> {
        (self as &RefCell<Entity>).borrow()
    }
    fn borrow_mut(&self) -> RefMut<Entity> {
        (self as &RefCell<Entity>).borrow_mut()
    }
    fn update(&self, dt: f64, world: &World, effect_manager: &mut EffectManager) {
        self.sword_update(dt,world,effect_manager);
        self.gun_update(dt,world,effect_manager);
        self.borrow_mut().body.update(dt);
    }
}

impl Entity for Character {
    fn body(&self) -> &Body {
        &self.body
    }
    fn mut_body(&mut self) -> &mut Body {
        &mut self.body
    }
    fn render(&self, frame_manager: &mut FrameManager) {
        use std::f64::consts::*;
        match self.gun.gun_type {
            GunType::Rifle => frame_manager.draw_animation(self.body.x,self.body.y,self.aim,Animation::CharacterRifle),
            GunType::Shotgun => frame_manager.draw_animation(self.body.x,self.body.y,self.aim,Animation::CharacterShotgun),
            GunType::Sniper => frame_manager.draw_animation(self.body.x,self.body.y,self.aim,Animation::CharacterSniper),
            GunType::None => (),
        }
        let delta = (self.body.angle - self.aim).abs();
        if delta <= FRAC_PI_4 {
            frame_manager.draw_animation(self.body.x,self.body.y,self.aim,Animation::CharacterCloakUnfold);
        } else if delta <= 3.*FRAC_PI_4 {
            if self.body.angle > self.aim {
                frame_manager.draw_animation(self.body.x,self.body.y,self.aim,Animation::CharacterCloakLeft);
            } else {
                frame_manager.draw_animation(self.body.x,self.body.y,self.aim,Animation::CharacterCloakRight);

            }
        } else {
            frame_manager.draw_animation(self.body.x,self.body.y,self.aim,Animation::CharacterCloakFold);
        }
        // self.body.render(color::RED,frame_manager);
    }
}

struct Sword {
    recover: f64,
    do_attack: bool
}

impl Sword {
    fn new() -> Sword {
        Sword {
            recover: 0.,
            do_attack: false,
        }
    }
}

trait SwordManager {
    fn sword_update(&self, dt: f64, world: &World, effect_manager: &mut EffectManager);
}

impl SwordManager for RefCell<Character> {
    fn sword_update(&self, dt: f64, world: &World, effect_manager: &mut EffectManager) {
        use std::f64::consts::{PI, FRAC_PI_2};

        {
            let sword = &mut self.borrow_mut().sword;
            if sword.recover > 0. {
                sword.recover = (sword.recover - dt).max(0.);
            }
        }

        if self.borrow().sword.do_attack && self.borrow().sword.recover == 0. {
            self.borrow_mut().sword.recover = SWORD_RECOVER;

            let (id,x,y,aim) = {
                let this = self.borrow();
                let body = this.body();
                (body.id, body.x, body.y, minus_pi_pi(self.aim()))
            };
            let loc = Location {
                up: y + SWORD_LENGTH,
                down: y - SWORD_LENGTH,
                left: x - SWORD_LENGTH,
                right: x + SWORD_LENGTH,
            };


            world.apply_locally(!group::ARMORY, &loc, &mut |entity: &mut Entity| {
                let attack = {
                    let body = entity.body();
                    if body.id != id && body.in_circle([x,y],SWORD_LENGTH) {
                        if aim == PI {
                            body.left() <= x
                        } else if aim > FRAC_PI_2 {
                            let t_x = body.left() - x;
                            let t_y = body.up() - y;
                            let a = aim - FRAC_PI_2;
                            t_y >= a * t_x
                        } else if aim == FRAC_PI_2 {
                            body.up() >= y
                        } else if aim > 0. {
                            let t_x = body.right() - x;
                            let t_y = body.up() - y;
                            let a = aim - FRAC_PI_2;
                            t_y >= a * t_x
                        } else  if aim == 0. {
                            body.right() >= x
                        } else if aim > -FRAC_PI_2 {
                            let t_x = body.right() - x;
                            let t_y = body.down() - y;
                            let a = aim - FRAC_PI_2;
                            t_y >= a * t_x
                        } else if aim == -FRAC_PI_2 {
                            body.down() <= y
                        } else {
                            let t_x = body.left() - x;
                            let t_y = body.down() - y;
                            let a = aim - FRAC_PI_2;
                            t_y >= a * t_x
                        }
                    } else {
                        false
                    }
                };

                if attack {
                    entity.mut_body().damage(SWORD_DAMAGE);
                }
            });

            effect_manager.add(Effect::SwordAttack(Position::new(x,y),self.borrow().aim));
        }
        self.borrow_mut().sword.do_attack = false;
    }
}

#[derive(Clone,Copy,PartialEq)]
pub enum GunType {
    None,
    Rifle,
    Shotgun,
    Sniper,
}

struct Gun {
    gun_type: GunType,
    reloading: f64,
    shooting: bool,
    ammo: u64,
}

impl Gun {
    pub fn new() -> Gun {
        Gun {
            gun_type: GunType::Rifle,
            shooting: false,
            reloading: 0.,
            ammo: 10000000,
        }
    }

    pub fn time_to_reload(&mut self) -> f64 {
        match self.gun_type {
            GunType::Sniper => SNIPER_RELOADING_TIME,
            GunType::Shotgun => SHOTGUN_RELOADING_TIME,
            GunType::Rifle => RIFLE_RELOADING_TIME,
            GunType::None => 0.,
        }
    }
}

trait GunManager {
    fn gun_shoot(&self, world: &World, effect_manager: &mut EffectManager);
    fn gun_update(&self, dt: f64, world: &World, effect_manager: &mut EffectManager);
}

impl GunManager for RefCell<Character> {
    fn gun_shoot(&self,world: &World, effect_manager: &mut EffectManager) {
        let (id,x,y,aim,gun_type) = {
            let this = self.borrow();
            (this.body.id,this.body.x,this.body.y,this.aim,this.gun.gun_type)
        };
        match gun_type {
            GunType::Sniper => {
                let mut length = SNIPER_LENGTH;
                world.raycast(!group::ARMORY,x,y,aim,SNIPER_LENGTH, &mut |entity,min,_| {
                    let body = entity.mut_body();
                    if body.id != id {
                        if body.group & group::WALL_KIND != 0 {
                            length = min;
                            true
                        } else {
                            body.damage(SNIPER_DAMAGE);
                            false
                        }
                    } else {
                        false
                    }
                });
                effect_manager.add(Effect::SniperShoot(Line::new(x,y,aim,length)));
            },
            GunType::Shotgun => {
                let range = Range::new(-SHOTGUN_MAX_DELTA_ANGLE,SHOTGUN_MAX_DELTA_ANGLE);
                let mut rng = rand::thread_rng();
                let mut shoots = vec!();
                for _ in 0..SHOTGUN_SHOOT_NUMBER {
                    let aim = aim + range.ind_sample(&mut rng);
                    let mut length = SHOTGUN_LENGTH;
                    world.raycast(!group::ARMORY,x,y,aim,SHOTGUN_LENGTH, &mut |entity,min,_| {
                        let body = entity.mut_body();
                        if body.id != id {
                            body.damage(SHOTGUN_DAMAGE);
                            length = min;
                            true
                        } else {
                            false
                        }
                    });
                    shoots.push(Line::new(x,y,aim,length));
                }
                effect_manager.add(Effect::ShotgunShoot(shoots));
            },
            GunType::Rifle => {
                let range = Range::new(-RIFLE_MAX_DELTA_ANGLE,RIFLE_MAX_DELTA_ANGLE);
                let mut rng = rand::thread_rng();
                let aim = aim + range.ind_sample(&mut rng);
                let mut length = RIFLE_LENGTH;
                world.raycast(!group::ARMORY,x,y,aim,RIFLE_LENGTH, &mut |entity,min,_| {
                    let body = entity.mut_body();
                    if body.id != id {
                        body.damage(RIFLE_DAMAGE);
                        length = min;
                        true
                    } else {
                        false
                    }
                });
                effect_manager.add(Effect::RifleShoot(Line::new(x,y,aim,length)));
            },
            GunType::None => (),
        }
    }

    fn gun_update(&self, dt: f64, world: &World, effect_manager: &mut EffectManager) {
        // pickup gun
        // let pickup = self.borrow().gun.pickup;
        // if pickup {
        //     let mut item = None;
        //     let location = self.borrow().body.location();
        //     world.apply_locally(super::group::ARMORY,&location,&mut |entity: &mut Entity| {
        //         if let None = item {
        //             let body = entity.mut_body();
        //             if  body.items.len() > 0 {
        //                 item = Some(body.items.remove(0));
        //             }
        //         }
        //     });
        //     let mut this = self.borrow_mut();
        //     this.gun.pickup = false;
        let pickup = self.borrow().body.items.len() != 0;
        if pickup {
            let mut this = self.borrow_mut();
            let item = this.body.items.pop().unwrap();
            this.body.items = Vec::new();
            match item {
                Item::Rifle(ammo) => {
                    this.gun.ammo = ammo;
                    this.gun.gun_type = GunType::Rifle;
                },
                Item::Shotgun(ammo) => {
                    this.gun.ammo = ammo;
                    this.gun.gun_type = GunType::Shotgun;
                },
                Item::Sniper(ammo) => {
                    this.gun.ammo = ammo;
                    this.gun.gun_type = GunType::Sniper;
                },
            }
            this.gun.reloading = this.gun.time_to_reload();
        }

        // shoot
        let shoot = {
            let mut this = self.borrow_mut();
            if this.gun.ammo > 0 {
                if this.gun.reloading > 0. {
                    if this.gun.shooting {
                        this.gun.reloading -= dt;
                    } else {
                        let t = this.gun.reloading - dt;
                        this.gun.reloading = t.max(0.);
                    }
                    false
                } else if this.gun.shooting {
                    this.gun.ammo -= 1;
                    this.gun.reloading += this.gun.time_to_reload();
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        if shoot {
            self.gun_shoot(world,effect_manager);
        }
    }
}

