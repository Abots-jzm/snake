use macroquad::prelude::*;

mod bot;
mod game;
mod snake;

use crate::game::Game;

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake".to_owned(),
        window_width: 500,
        window_height: 500,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        clear_background(BLACK);
        game.handle_input();
        game.update(get_frame_time());
        game.render();
        next_frame().await
    }
}
