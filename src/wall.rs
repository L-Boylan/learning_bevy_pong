use bevy::prelude::*;
use crate::Collider;

#[derive(Component)]
pub struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}
pub enum WallLocation{
    Left,
    Right,
    Top,
    Bottom,
}

const LEFT_WALL: f32 = -450.0;
const RIGHT_WALL: f32 = 450.0;
const TOP_WALL: f32 = 350.0;
const BOTTOM_WALL: f32 = -350.0;
const WALL_THICKNESS: f32 = 10.0;

impl WallLocation {
    pub fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.0),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.0),
            WallLocation::Bottom => Vec2::new(0.0, BOTTOM_WALL),
            WallLocation::Top => Vec2::new( 0.0, TOP_WALL)
        }
    }

    pub fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;

        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    pub fn new(location: WallLocation) -> WallBundle{
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.8, 0.8),
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}