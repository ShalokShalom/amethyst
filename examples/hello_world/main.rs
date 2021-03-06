//! The simplest Amethyst example.

extern crate amethyst;

use amethyst::prelude::*;

struct Example;

impl State for Example {
    fn on_start(&mut self, _: &mut World) {
        println!("Begin!");
    }

    fn update(&mut self, _: &mut World) -> Trans {
        println!("Hello from Amethyst!");
        Trans::Quit
    }

    fn on_stop(&mut self, _: &mut World) {
        println!("End!");
    }
}

fn main() {
    let mut game = Application::new("./", Example).expect("Fatal error");
    game.run();
}
