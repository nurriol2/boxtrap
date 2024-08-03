use bevy::prelude::*;

// Resources
#[derive(Resource)]
struct GreetTimer(Timer);

// Components
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// Systems
fn hello_world_system() {
    // Simple system
    println!("hello world!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Alice".to_string())));
    commands.spawn((Person, Name("Bob".to_string())));
    commands.spawn((Person, Name("Charlie".to_string())));
}

// Queries
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // Update the timer with the time elapsed since last update
    // If the update caused the timer to finish, print
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Alice" {
            name.0 = "Alice L.".to_string();
            break;
        }
    }
}

// Plugins
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people); // Register system on the Startup scheduler
        app.add_systems(
            // Systems run in parallel
            Update,
            (
                //hello_world_system,
                (update_people, greet_people).chain(), // Chain specifies the order
            ),
        );
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
