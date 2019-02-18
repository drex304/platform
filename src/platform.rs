use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
    SpriteSheetHandle, Texture, TextureMetadata
};

use crate::components;

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;
pub const PLAYER_WIDTH: f32 = 5.0;
pub const PLAYER_HEIGHT: f32 = 5.0;
pub const _ENEMY_WIDTH: f32 = 5.0;
pub const _ENEMY_HEIGHT: f32 = 5.0;
pub const WALL_WIDTH: f32 = 5.0;
pub const WALL_HEIGHT: f32 = 5.0;
pub const _BULLET_WIDTH: f32 = 1.0;
pub const _BULLETWALL_HEIGHT: f32 = 1.0;

pub struct Platform;

pub struct Player {
    pub width: f32,
    pub height: f32,
    pub correction_x: f32,
    pub correction_y: f32,
}

pub struct _Enemy {
    pub width: f32,
    pub height: f32,
}

pub struct Wall {
    pub width: f32,
    pub height: f32,
}

pub struct _Bullet {
    pub width: f32,
    pub height: f32,
}

impl Player {
    fn new() -> Player {
        Player {
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT,
            correction_x: 0.0,
            correction_y: 0.0,
        }
    }
}

impl _Enemy {
    fn _new() -> _Enemy {
        _Enemy {
            width: _ENEMY_WIDTH,
            height: _ENEMY_HEIGHT,
        }
    }
}

impl Wall {
    fn new() -> Wall {
        Wall {
            width: WALL_WIDTH,
            height: WALL_HEIGHT,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl Component for _Enemy {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Wall {
    type Storage = DenseVecStorage<Self>;
}

impl SimpleState for Platform {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        //world.register::<Enemy>();

        let sprite_sheet = load_sprite_sheet(world);

        initialise_camera(world);
        initialise_player(world, sprite_sheet.clone());
        //initialise_enemy(world, sprite_sheet.clone());
        initialise_walls(world, sprite_sheet.clone());
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

fn initialise_player(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    // position the player at the middle of the screen
    let x = ARENA_WIDTH / 2.0 - PLAYER_WIDTH / 2.0;
    let y = ARENA_HEIGHT / 2.0 - PLAYER_HEIGHT / 2.0;
    let mut initial_transform = Transform::default();
    initial_transform.set_xyz(x, y, 0.0);

    let sprite_renderer = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: 1,
    };

    world
        .create_entity()
        .with(sprite_renderer)
        .with(Player::new())
        .with(components::Velocity::default())
        .with(components::Gravity::default())
        .with(initial_transform)
        .build();
}

fn _initialise_enemy(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    // position the enemy at the top left of the screen
    let x = 0.0 + _ENEMY_WIDTH / 2.0;
    let y = ARENA_HEIGHT - _ENEMY_HEIGHT / 2.0;
    let mut initial_transform = Transform::default();
    initial_transform.set_xyz(x, y, 0.0);

    let sprite_renderer = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_renderer)
        .with(_Enemy::_new())
        .with(initial_transform)
        .build();
}

fn initialise_walls(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    // add 20 wall panels to create a floor
    let y = 0.0 + WALL_HEIGHT / 2.0;
    for i in 0..20 {
        let x = (WALL_WIDTH * i as f32) + WALL_WIDTH / 2.0;
        let mut initial_transform = Transform::default();
        initial_transform.set_xyz(x, y, 0.0);

        let sprite_renderer = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 3,
        };

        world
            .create_entity()
            .with(sprite_renderer)
            .with(Wall::new())
            .with(initial_transform)
            .build();
    }
    // add two wall panels to create walls
    for i in [0.0f32, 19.0f32].iter() {
        let x = (WALL_WIDTH * i) + WALL_WIDTH / 2.0;
        let mut initial_transform1 = Transform::default();
        let mut initial_transform2 = Transform::default();
        initial_transform1.set_xyz(x, WALL_HEIGHT * 1.5, 0.0);
        initial_transform2.set_xyz(x, WALL_HEIGHT * 2.5, 0.0);

        let sprite_renderer = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 3,
        };

        world
            .create_entity()
            .with(sprite_renderer.clone())
            .with(Wall::new())
            .with(initial_transform1)
            .build();
        world
            .create_entity()
            .with(sprite_renderer)
            .with(Wall::new())
            .with(initial_transform2)
            .build();
    }
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a clonable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/platform_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/platform_spritesheet.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}
