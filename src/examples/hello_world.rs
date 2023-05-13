use bevy::{
    app::App,
    prelude::{Commands, Component, Plugin, Query, Res, ResMut, Resource, With},
    time::{Time, Timer, TimerMode},
    DefaultPlugins,
};
fn hello_world() {
    println!("hello world!");
}

#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}
#[derive(Resource)]
struct GreetTimer(Timer);
// fn greet_people( query: Query<&Name, With<Person>>) {
//     for name in &query {
//         println!("hello {}!", name.0);
//     }
// }

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        // app.add_startup_system(add_people)
        // .add_system(hello_world)
        // .add_system(greet_people);

        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}

pub fn hello_wrold_main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_people)
        // .add_system(hello_world)
        // .add_system(greet_people)
        .add_plugin(HelloPlugin)
        .run();
}
