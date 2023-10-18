use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);
#[derive(Component)]
pub struct Collider;
#[derive(Event, Default)]
pub struct ColliderEvent;
#[derive(Component)]
pub struct Ball;

pub const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
pub const BALL_SPEED: f32 = 400.0;

pub fn draw_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
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

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>){
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    }
}

pub fn check_for_collisions(
    mut commands: Commands,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
    mut collision_events: EventWriter<ColliderEvent>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();
}