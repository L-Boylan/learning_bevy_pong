use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::{Collider, wall};
use crate::ball::{Ball, Velocity};
use crate::paddle::{Player1, Player2};
use crate::wall::WallBundle;

const PADDLE_SIZE: Vec2 = Vec2::new(30.0, 100.0);
pub const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
pub const BALL_SPEED: f32 = 400.0;

pub fn draw_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((Player1, Collider, SpriteBundle{
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(PADDLE_SIZE),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-480.0, 0.0, 0.0)),
        ..default()
    }));

    commands.spawn((Player2, Collider, SpriteBundle{
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(PADDLE_SIZE),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(480.0, 0.0, 0.0)),
        ..default()
    }));

    commands.spawn(WallBundle::new(wall::WallLocation::Left));
    commands.spawn(WallBundle::new(wall::WallLocation::Right));
    commands.spawn(WallBundle::new(wall::WallLocation::Top));
    commands.spawn(WallBundle::new(wall::WallLocation::Bottom));

    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(12.0).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    },
    Ball,
    Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED)
    ));
}