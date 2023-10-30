use bevy::prelude::*;
use crate::Collider;

#[derive(Component)]
pub struct Player1;
#[derive(Component)]
pub struct Player2;

const PADDLE_SPEED: f32 = 500.0;
const PADDLE_SIZE: Vec2 = Vec2::new(30.0, 100.0);

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

    let top_bound = 350.0 - PADDLE_SIZE.y / 2.0;
    let bottom_bound = -350.0 +PADDLE_SIZE.y / 2.0;

    let new_paddle_position = paddle_transform.translation.y + direction * PADDLE_SPEED * time_step.period.as_secs_f32();

    paddle_transform.translation.y = new_paddle_position.clamp( bottom_bound, top_bound);
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

    let top_bound = 350.0 - PADDLE_SIZE.y / 2.0;
    let bottom_bound = -350.0 +PADDLE_SIZE.y / 2.0;

    let new_paddle_position = paddle_transform.translation.y + direction * PADDLE_SPEED * time_step.period.as_secs_f32();

    paddle_transform.translation.y = new_paddle_position.clamp(bottom_bound, top_bound);
}

