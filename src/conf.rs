use configuration::BitflagU32;

pub type Effect = [(String, u32);4];
pub type Music = [String;1];
pub type Dimension = [u32;2];
pub type Array4F32 = [f32;4];
pub type VecU8 = Vec<u8>;

pub mod snd_effect {
    pub const RIFLE_SHOOT_ZERO: usize = 0;
    pub const RIFLE_SHOOT_ONE: usize = 1;
    pub const RIFLE_SHOOT_LOTS: usize = 2;
    pub const RIFLE_RELOAD: usize = 3;
}
pub mod music {
    pub const BACKGROUND: usize = 0;
}

fn config_constraint(conf: &Config) -> Result<(),String> {
    if conf.keys.up.len() == 0
       || conf.keys.down.len() == 0
       || conf.keys.left.len() == 0
       || conf.keys.right.len() == 0 {
           return Err("ERROR: configuration file invalid: keys mustn't be empty".into());
    }

    Ok(())
}

configure!(
    file = "config.toml";
    debug_file = "config.toml";

    constraint = config_constraint;

    general: {
        number_of_thread: t usize,
    },
    keys: {
        up: t VecU8,
        down: t VecU8,
        left: t VecU8,
        right: t VecU8,
        quit: t VecU8,
    },
    physic: {
        rate: t f32,
        unit: t f32,
    },
    entities: {
        ball_group: t BitflagU32,
        ball_mask: t BitflagU32,
        ball_killer_mask: t BitflagU32,
        ball_radius: t f32,
        ball_velocity: t f32,
        ball_time: t f32,
        ball_weight: t f32,
        ball_color: e String [base5,base4,base3,base2,base1,yellow,orange,red,magenta,violet,blue,cyan,green],

        laser_group: t BitflagU32,
        laser_mask: t BitflagU32,
        laser_killer_mask: t BitflagU32,
        laser_radius: t f32,
        laser_color: e String [base5,base4,base3,base2,base1,yellow,orange,red,magenta,violet,blue,cyan,green],

        column_group: t BitflagU32,
        column_mask: t BitflagU32,
        column_radius: t f32,
        column_color: e String [base5,base4,base3,base2,base1,yellow,orange,red,magenta,violet,blue,cyan,green],

        char_group: t BitflagU32,
        char_mask: t BitflagU32,
        char_radius: t f32,
        char_velocity: t f32,
        char_time: t f32,
        char_weight: t f32,
        char_color: e String [base5,base4,base3,base2,base1,yellow,orange,red,magenta,violet,blue,cyan,green],

        wall_group: t BitflagU32,
        wall_mask: t BitflagU32,
        wall_radius: t f32,
        wall_color: e String [base5,base4,base3,base2,base1,yellow,orange,red,magenta,violet,blue,cyan,green],

        monster_vision_mask: t BitflagU32,
        monster_killer_mask: t BitflagU32,
        monster_group: t BitflagU32,
        monster_mask: t BitflagU32,
        monster_vision_time: t f32,
        monster_radius: t f32,
        monster_velocity: t f32,
        monster_time: t f32,
        monster_weight: t f32,
        monster_color: e String [base5,base4,base3,base2,base1,yellow,orange,red,magenta,violet,blue,cyan,green],

        portal_end_color: e String [base5,base4,base3,base2,base1,yellow,orange,red,magenta,violet,blue,cyan,green],
        portal_start_color: e String [base5,base4,base3,base2,base1,yellow,orange,red,magenta,violet,blue,cyan,green],
    },
    levels: {
        dir: t String,
        first_level: t String,
        common: t String,
    },
    // audio: {
    //     channels: t i32,
    //     sample_rate: t f64,
    //     frames_per_buffer: t u32,
    //     effect_dir: t String,
    //     music_dir: t String,
    //     global_volume: t f32,
    //     music_volume: t f32,
    //     effect_volume: t f32,
    //     distance_model: e String [linear,pow2],
    //     distance_model_min: t f64,
    //     distance_model_max: t f64,
    //     music_loop: t bool,
    //     effect: t Effect,
    //     music: t Music,
    //     check_level: e String [always,debug,never],
    // },
    window: {
        dimension: t Dimension,
        vsync: t bool,
        multisampling: t u16,
    },
    graphics: {
        base03: t Array4F32,
        base02: t Array4F32,
        base01: t Array4F32,
        base00: t Array4F32,
        base0: t Array4F32,
        base1: t Array4F32,
        base2: t Array4F32,
        base3: t Array4F32,
        yellow: t Array4F32,
        orange: t Array4F32,
        red: t Array4F32,
        magenta: t Array4F32,
        violet: t Array4F32,
        blue: t Array4F32,
        cyan: t Array4F32,
        green: t Array4F32,
        mode: e String [light,dark],
        luminosity: t f32,
        circle_precision: t usize,
        font_precision: t u32,
        font_file: t String,
        font_ratio: t f32,
    },
    camera: {
        zoom: t f32,
    },
    event_loop: {
        ups: t u64,
        max_fps: t u64,
    },
);

