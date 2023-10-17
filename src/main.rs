mod ball;
mod paddle;
use bevy::prelude::*;

pub struct HelloPlugin;
#[derive(Resource)]
struct GreetTimer(Timer);
#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);

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

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App){
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, (add_people, paddle::draw_paddle, ball::draw_ball))
            .add_systems(FixedUpdate, (paddle::move_paddle_player1, paddle::move_paddle_player2))
            .add_systems(Update, greet_people);
    }
}