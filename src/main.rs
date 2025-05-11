use macroquad::prelude::*;
use snake::SNAKE_SPEED;

mod snake;

use crate::snake::Snake;

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
    let mut snake = Snake::spawn_on_map(5, 5, 5);
    let mut step_timer: f32 = 0.;

    loop {
        clear_background(BLACK);
        let delta_time = get_frame_time();

        snake.listen_for_input();
        step_timer += delta_time;
        if step_timer >= 1. / SNAKE_SPEED {
            snake.step();
            step_timer = 0.;
        }
        snake.draw(step_timer);

        next_frame().await
    }
}
