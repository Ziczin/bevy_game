use bevy::prelude::*;


#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Scientist;

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Resource)]
struct ScienceTimer(Timer);

#[derive(Resource)]
struct MarriegeTimer(Timer);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string()), Scientist));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>
) {
    let mut idx: i32 = 0;
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("{} Hello {}!", idx, name.0);
            idx += 1;
        }
    }
}

fn update_people(
    time: Res<Time>,
    mut timer: ResMut<MarriegeTimer>,
    mut query: Query<&mut Name, With<Person>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut name in &mut query {
            if name.0 == "Elaina Proctor" {
                name.0 = "Elaina Hume".to_string();
                break;
            }
        }
    }
}

fn scientist_do_science(
    time: Res<Time>,
    mut timer: ResMut<ScienceTimer>,
    query: Query<&Name, With<Scientist>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("{} do some science!", name.0);
        }
    }
}

pub struct HelloWorldPlugin;

impl Plugin for HelloWorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
        app.insert_resource(ScienceTimer(Timer::from_seconds(2.5, TimerMode::Repeating)));
        app.insert_resource(MarriegeTimer(Timer::from_seconds(5.0, TimerMode::Once)));

        app.add_systems(Startup, add_people);
        app.add_systems(Update, (scientist_do_science, update_people, greet_people));
    }
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(HelloWorldPlugin)
        .run();
}
