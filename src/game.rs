use macroquad::prelude::*;

use crate::snake::Snake;
use crate::snake::SNAKE_SPEED;

pub struct Game {
    score: u32,
    pub is_over: bool,
    pub snake: Snake,
    step_timer: f32,
}

impl Game {
    pub fn new() -> Self {
        Game {
            score: 0,
            is_over: false,
            snake: Snake::spawn_on_map(5, 5, 5),
            step_timer: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.is_over {
            return;
        }

        self.step_timer += delta_time;

        if self.step_timer >= 1. / SNAKE_SPEED {
            self.snake.step();
            self.check_for_death();
            self.step_timer = 0.;
        }
    }

    pub fn render(&self) {
        self.snake.draw(self.step_timer);
        self.draw_score();
        if self.is_over {
            self.draw_game_over();
        }
    }

    fn draw_score(&self) {
        let score_text = format!("Score: {}", self.score);
        draw_text(&score_text, 10.0, 20.0, 20.0, WHITE);
    }

    fn draw_game_over(&self) {
        let game_over_text = "Game Over! Press Enter to restart or Escape to exit.";
        //center text based on screen size and place towards the bottom
        let screen_width = screen_width();
        let screen_height = screen_height();
        let text_width = measure_text(game_over_text, None, 20, 1.0).width;
        let text_x = (screen_width - text_width) / 2.0;
        let text_y = screen_height - 50.0; // 50 pixels from the bottom
        draw_rectangle(
            0.0,
            0.0,
            screen_width,
            screen_height,
            Color::new(0.0, 0.0, 0.0, 0.5),
        );
        draw_text(&game_over_text, text_x, text_y, 20.0, WHITE);
    }

    pub fn handle_input(&mut self) {
        if self.is_over {
            if is_key_pressed(KeyCode::Enter) {
                self.reset();
            } else if is_key_pressed(KeyCode::Escape) {
                std::process::exit(0);
            }
            return;
        }

        self.snake.handle_input();
    }

    fn check_for_death(&mut self) {
        if self.snake.is_dead() {
            self.game_over();
        }
    }

    fn reset(&mut self) {
        self.score = 0;
        self.is_over = false;
        self.snake = Snake::spawn_on_map(5, 5, 5);
        self.step_timer = 0.0;
    }

    fn game_over(&mut self) {
        self.is_over = true;
    }
}
