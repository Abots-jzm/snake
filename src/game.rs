use macroquad::prelude::*;

use crate::snake::{Snake, CELL_GAP};
use crate::snake::{CELL_SIZE, SNAKE_SPEED};

pub struct Game {
    score: u32,
    is_over: bool,
    snake: Snake,
    step_timer: f32,
    open_cells: Vec<(usize, usize)>,
    apple: (usize, usize),
}

impl Game {
    pub fn new() -> Self {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let snake = Snake::spawn_on_map(5, 5, 4);
        // open_cells should be a vector of tuples (x, y) representing the available cells for the apple i.e entire map - snake cells
        let mut open_cells = Vec::new();
        for x in 0..(screen_width as usize / CELL_SIZE as usize) {
            for y in 0..(screen_height as usize / CELL_SIZE as usize) {
                if !snake.segments.iter().any(|s| s.cur == (x, y)) {
                    open_cells.push((x, y));
                }
            }
        }
        rand::srand(macroquad::miniquad::date::now() as _);
        let apple_index = rand::gen_range(0, open_cells.len());
        let apple = open_cells[apple_index];

        Game {
            score: 0,
            is_over: false,
            snake,
            step_timer: 0.0,
            open_cells,
            apple,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.is_over {
            return;
        }

        self.step_timer += delta_time;

        if self.step_timer >= 1. / SNAKE_SPEED {
            let (head, tail) = self.snake.step();
            // remove head from open_cells
            self.open_cells.retain(|&cell| cell != head);
            // add tail to open_cells

            if self.snake.is_eating(self.apple) {
                self.score += 1;
                self.snake.grow();

                // Spawn a new apple
                if !self.open_cells.is_empty() {
                    self.apple = self.spawn_apple(&self.open_cells);
                } else {
                    // No more open cells, game over
                    self.is_over = true;
                }
            } else {
                self.open_cells.push(tail);
            }

            self.check_for_death();
            self.step_timer = 0.;
        }
    }

    pub fn render(&self) {
        self.snake.draw(self.step_timer);
        self.draw_apple();
        self.draw_score();
        if self.is_over {
            self.draw_game_over();
        }
    }

    fn spawn_apple(&self, open_cells: &[(usize, usize)]) -> (usize, usize) {
        let apple_index = rand::gen_range(0, open_cells.len());
        open_cells[apple_index]
    }

    fn draw_apple(&self) {
        // Calculate cell coordinates
        let apple_x = self.apple.0 as f32 * CELL_SIZE;
        let apple_y = self.apple.1 as f32 * CELL_SIZE;

        // Calculate center of the cell accounting for gap
        let center_x = apple_x + CELL_SIZE / 2.0;
        let center_y = apple_y + CELL_SIZE / 2.0;

        // Calculate radius of the apple
        let radius = (CELL_SIZE - CELL_GAP) / 2.0;

        // Draw the apple as a circle
        draw_circle(center_x, center_y, radius, RED);
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
            self.is_over = true;
        }
    }

    fn reset(&mut self) {
        let screen_width = screen_width() as usize;
        let screen_height = screen_height() as usize;

        let snake = Snake::spawn_on_map(5, 5, 4);
        // open_cells should be a vector of tuples (x, y) representing the available cells for the apple i.e entire map - snake cells
        let mut open_cells = Vec::new();
        for x in 0..(screen_width / CELL_SIZE as usize) {
            for y in 0..(screen_height / CELL_SIZE as usize) {
                if !snake.segments.iter().any(|s| s.cur == (x, y)) {
                    open_cells.push((x, y));
                }
            }
        }
        rand::srand(macroquad::miniquad::date::now() as _);
        let apple_index = rand::gen_range(0, open_cells.len());
        let apple = open_cells[apple_index];

        self.score = 0;
        self.is_over = false;
        self.step_timer = 0.0;
        self.open_cells = open_cells;
        self.apple = apple;
        self.snake = snake;
    }
}
