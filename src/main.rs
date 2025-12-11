use bevy::prelude::*;


// components
#[derive(Component)]
struct Cat;

#[derive(Component)]
struct Name(String);

// data
const CAT_NAMES: &[&str; 2] = &["Boo", "Hartley"];
// functions
fn greet_cats(query: Query<&Name, With<Cat>>) {
    for name in query {
        println!("omg hi {}!", name.0);
    }
}

fn add_cats(mut commands: Commands) {
    for name in CAT_NAMES {
        commands.spawn((Cat, Name(name.to_string())));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_cats)
        .add_systems(Update, greet_cats)
        .run();
}
