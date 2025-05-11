use macroquad::prelude::*;

pub const CELL_SIZE: f32 = 15.;
pub const SNAKE_SPEED: f32 = 20.;

pub struct Snake {
    segments: Vec<SnakeSegment>,
    cur_directions: (i32, i32),
    future_directions: (i32, i32),
}

struct SnakeSegment {
    cur: (usize, usize),
    prev: (usize, usize),
}

impl Snake {
    pub fn spawn_on_map(x: usize, y: usize, length: usize) -> Self {
        let mut segments = Vec::new();
        for i in 0..length {
            segments.push(SnakeSegment {
                cur: (x - i, y),
                prev: (x - i, y),
            });
        }

        Self {
            segments,
            cur_directions: (1, 0), // Start moving to the right
            future_directions: (1, 0),
        }
    }

    pub fn draw(&self, snake_timer: f32) {
        let ratio = snake_timer * SNAKE_SPEED;

        // First draw all regular segments
        self.segments.iter().enumerate().for_each(|(i, segment)| {
            let draw_location_x: f32;
            let draw_location_y: f32;

            if i == 0 {
                draw_location_x =
                    (segment.cur.0 as f32 + self.cur_directions.0 as f32 * ratio) * CELL_SIZE;
                draw_location_y =
                    (segment.cur.1 as f32 + self.cur_directions.1 as f32 * ratio) * CELL_SIZE;
            } else {
                let future_pos = self.segments[i - 1].cur;
                draw_location_x = (segment.cur.0 as f32 * (1.0 - ratio)
                    + future_pos.0 as f32 * ratio)
                    * CELL_SIZE;
                draw_location_y = (segment.cur.1 as f32 * (1.0 - ratio)
                    + future_pos.1 as f32 * ratio)
                    * CELL_SIZE;
            }

            draw_rectangle(
                draw_location_x,
                draw_location_y,
                CELL_SIZE,
                CELL_SIZE,
                GREEN,
            );

            // Add corner pieces for turns to fill gaps
            if i > 0 {
                let prev_segment = &self.segments[i - 1];

                // Check if this is a turn by comparing the current segment's position with previous one
                let dx = prev_segment.cur.0 as i32 - segment.cur.0 as i32;
                let dy = prev_segment.cur.1 as i32 - segment.cur.1 as i32;

                // If there's a turn, draw a connecting piece
                if (dx != 0 && dy != 0)
                    || (i > 1
                        && ((prev_segment.cur.0 != prev_segment.prev.0)
                            || (prev_segment.cur.1 != prev_segment.prev.1)))
                {
                    // Draw corner piece at the previous segment's position
                    draw_rectangle(
                        prev_segment.cur.0 as f32 * CELL_SIZE,
                        prev_segment.cur.1 as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        GREEN,
                    );
                }
            }
        });
    }

    pub fn listen_for_input(&mut self) {
        if is_key_down(KeyCode::Up) {
            self.future_directions = (0, -1);
        } else if is_key_down(KeyCode::Down) {
            self.future_directions = (0, 1);
        } else if is_key_down(KeyCode::Left) {
            self.future_directions = (-1, 0);
        } else if is_key_down(KeyCode::Right) {
            self.future_directions = (1, 0);
        }
    }

    pub fn step(&mut self) {
        // Update the current direction if the future direction is not opposite
        if self.cur_directions.0 + self.future_directions.0 != 0
            || self.cur_directions.1 + self.future_directions.1 != 0
        {
            self.cur_directions = self.future_directions;
        }

        // Save the current positions before moving
        for segment in &mut self.segments {
            segment.prev = segment.cur;
        }

        // Move the head of the snake
        let head = &mut self.segments[0];
        head.cur = (
            (head.cur.0 as i32 + self.cur_directions.0) as usize,
            (head.cur.1 as i32 + self.cur_directions.1) as usize,
        );

        // Move the body segments
        for i in 1..self.segments.len() {
            self.segments[i].cur = self.segments[i - 1].prev;
        }
    }
}
