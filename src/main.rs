use bevy::prelude::*;

mod ball;
mod paddle;
mod wall;
mod draw;

pub struct HelloPlugin;
#[derive(Resource)]
struct GreetTimer(Timer);
#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);
#[derive(Component)]
pub struct Collider;
#[derive(Event, Default)]
pub struct CollisionEvent;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App){
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
            .add_event::<CollisionEvent>()
            .add_systems(Startup,  draw::draw_objects)
            .add_systems(FixedUpdate, (
                ball::check_for_collisions,
                ball::apply_velocity.before(ball::check_for_collisions),
                paddle::move_paddle_player1.before(ball::check_for_collisions).after(ball::apply_velocity),
                paddle::move_paddle_player2.before(ball::check_for_collisions).after(ball::apply_velocity),
            ));
    }
}