//Use of extern crate keywords are not needed since new updates of Rust
mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use draw::to_coord_u32;

const BACK_COLOR: Color = [0.5,0.5,0.5,1.0];

fn main() {
    let (width, height) = (30, 30);
    let mut window: PistonWindow = WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut game: Game = Game::new(width, height);
    //event loop below -> ignore no events a.k.a None option via while let binding
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        } 
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}