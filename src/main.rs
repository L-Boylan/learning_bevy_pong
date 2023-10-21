use bevy::prelude::*;
use crate::wall::WallBundle;

mod ball;
mod paddle;
mod wall;

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

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Bobby Brown".to_string())));
    commands.spawn((Person, Name("Johnny Brown".to_string())));
    commands.spawn((Person, Name("Betty Brown".to_string())));
    commands.spawn(Name("Boomer".to_string()));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello {}", name.0)
        }
    }
}

fn setup_walls(
    mut commands: Commands,
) {
    commands.spawn(WallBundle::new(wall::WallLocation::Left));
    commands.spawn(WallBundle::new(wall::WallLocation::Right));
    commands.spawn(WallBundle::new(wall::WallLocation::Top));
    commands.spawn(WallBundle::new(wall::WallLocation::Bottom));
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App){
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_event::<CollisionEvent>()
            .add_systems(Startup, (add_people, paddle::draw_paddle, ball::draw_ball, setup_walls))
            .add_systems(FixedUpdate, (
                ball::check_for_collisions,
                ball::apply_velocity.before(ball::check_for_collisions),
                paddle::move_paddle_player1.before(ball::check_for_collisions).after(ball::apply_velocity),
                paddle::move_paddle_player2.before(ball::check_for_collisions).after(ball::apply_velocity),
            ))
            .add_systems(Update, greet_people);
    }
}