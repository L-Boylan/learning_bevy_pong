use bevy::prelude::*;

#[derive(Component)]
pub struct Player1;
#[derive(Component)]
pub struct Player2;

const PADDLE_SPEED: f32 = 500.0;
pub fn draw_paddle(
    mut commands: Commands,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((Player1, SpriteBundle{
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-480.0, 0.0, 0.0)),
        ..default()
    }));

    commands.spawn((Player2, SpriteBundle{
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(480.0, 0.0, 0.0)),
        ..default()
    }));
}

pub fn move_paddle_player1(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player1>>,
    time_step: Res<FixedTime>,
){
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::S){
        direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::W){
        direction += 1.0;
    }

    let new_paddle_position = paddle_transform.translation.y + direction * PADDLE_SPEED * time_step.period.as_secs_f32();

    paddle_transform.translation.y = new_paddle_position;
}

pub fn move_paddle_player2(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player2>>,
    time_step: Res<FixedTime>,
){
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Up){
        direction += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down){
        direction -= 1.0;
    }

    let new_paddle_position = paddle_transform.translation.y + direction * PADDLE_SPEED * time_step.period.as_secs_f32();

    paddle_transform.translation.y = new_paddle_position;
}

