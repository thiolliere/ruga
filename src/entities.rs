use control::PlayerControl;
use weapons::{
    Rifle,
};
use physic::{
    PhysicState,
    PhysicType,
    PhysicForce,
    PhysicDynamic,
    Shape,
    CollisionBehavior,
};
use graphics::Color;
use specs;

pub struct Entities {
    pub character: Character,
    pub monster: Monster,
}

impl Entities {
    pub fn new() -> Self {
        Entities {
            character: Character,
            monster: Monster,
        }
    }
}

pub struct Character;

impl Character {
    pub fn build(&self, world: &specs::World, pos: [f32;2]) {
        world.create_now()
            .with::<PhysicState>(PhysicState::new(pos))
            .with::<PhysicDynamic>(PhysicDynamic)
            .with::<PhysicType>(PhysicType::new_movable(
                Shape::Circle(1.),
                CollisionBehavior::Persist,
                30.,
                0.2,
                1.))
            .with::<PhysicForce>(PhysicForce::new())
            .with::<Color>(Color::Yellow)
            .with::<PlayerControl>(PlayerControl)
            .with::<Rifle>(Rifle {
                rate: 1.,
                length: 20.,
                damage: 1.,
                shoot: false,
                recovery: 0.,
                ammo: 64,
                aim: 0.,
            })
            .build();
    }
}

pub struct Monster;

impl Monster {
    pub fn build(&self, world: &specs::World, pos: [f32;2]) {
        world.create_now()
            .with::<PhysicState>(PhysicState::new(pos))
            .with::<PhysicDynamic>(PhysicDynamic)
            .with::<PhysicType>(PhysicType::new_movable(
                Shape::Circle(1.),
                CollisionBehavior::Persist,
                30.,
                0.2,
                1.))
            .with::<PhysicForce>(PhysicForce::new())
            .with::<Color>(Color::Red)
            .build();
    }
}