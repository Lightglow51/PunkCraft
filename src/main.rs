use bevy::prelude::*;

fn hello_world() {
    println!("(!)");
}

fn main() {
    App::new()
        .add_system(hello_world)
        .run();
}