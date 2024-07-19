use std::{path};
use ggez::{ContextBuilder, event};
use ggez::conf::WindowMode;
use ggez_infinite_runner::MyGame;

fn main() {
    // Make a Context.
    let window_mode = WindowMode::default()
        .dimensions(1850.0, 1000.0);

    let resource_dir = path::PathBuf::from("./src/resources");

    let (mut ctx, event_loop) = ContextBuilder::new("Infinite Runner", "Michele Pulvirenti")
        .window_mode(window_mode)
        .add_resource_path(resource_dir)
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx).expect("Game error raised");

    // Run!
    event::run(ctx, event_loop, my_game);
}
