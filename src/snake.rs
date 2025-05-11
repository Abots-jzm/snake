use macroquad::prelude::*;

pub const CELL_SIZE: f32 = 15.;
// pub const SNAKE_SPEED: f32 = 20.;
pub const SNAKE_SPEED: f32 = 5.;
pub const CELL_GAP: f32 = 1.;

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
            let move_direction: (i32, i32);

            if i == 0 {
                draw_location_x =
                    (segment.cur.0 as f32 + self.cur_directions.0 as f32 * ratio) * CELL_SIZE;
                draw_location_y =
                    (segment.cur.1 as f32 + self.cur_directions.1 as f32 * ratio) * CELL_SIZE;
                move_direction = self.cur_directions;
            } else {
                let future_pos = self.segments[i - 1].cur;
                draw_location_x = (segment.cur.0 as f32 * (1.0 - ratio)
                    + future_pos.0 as f32 * ratio)
                    * CELL_SIZE;
                draw_location_y = (segment.cur.1 as f32 * (1.0 - ratio)
                    + future_pos.1 as f32 * ratio)
                    * CELL_SIZE;

                move_direction = (
                    future_pos.0 as i32 - segment.cur.0 as i32,
                    future_pos.1 as i32 - segment.cur.1 as i32,
                );
            }

            match move_direction {
                (1, 0) => draw_rectangle(
                    draw_location_x + CELL_GAP / 2.0,
                    draw_location_y + CELL_GAP / 2.0,
                    CELL_SIZE,
                    CELL_SIZE - CELL_GAP,
                    GREEN,
                ),
                (-1, 0) => draw_rectangle(
                    draw_location_x - CELL_GAP / 2.0,
                    draw_location_y + CELL_GAP / 2.0,
                    CELL_SIZE,
                    CELL_SIZE - CELL_GAP,
                    GREEN,
                ),
                (0, 1) => draw_rectangle(
                    draw_location_x + CELL_GAP / 2.0,
                    draw_location_y + CELL_GAP / 2.0,
                    CELL_SIZE - CELL_GAP,
                    CELL_SIZE,
                    GREEN,
                ),
                (0, -1) => draw_rectangle(
                    draw_location_x + CELL_GAP / 2.0,
                    draw_location_y - CELL_GAP / 2.0,
                    CELL_SIZE - CELL_GAP,
                    CELL_SIZE,
                    GREEN,
                ),
                _ => {}
            }

            // Add corner pieces for turns to fill gaps
            if i > 0 && i < self.segments.len() - 1 {
                let prev_segment = &self.segments[i - 1];
                let next_segment = &self.segments[i + 1];

                // A corner is formed when previous and next segments have different directions
                // Check if the x or y positions are different from both neighbors
                let is_corner = prev_segment.cur.0 != next_segment.cur.0
                    && prev_segment.cur.1 != next_segment.cur.1;

                if is_corner {
                    match move_direction {
                        (1, 0) => draw_rectangle(
                            (segment.cur.0 as f32 * CELL_SIZE) + CELL_GAP / 2.0,
                            (segment.cur.1 as f32 * CELL_SIZE) + CELL_GAP / 2.0,
                            CELL_SIZE,
                            CELL_SIZE - CELL_GAP,
                            GREEN,
                        ),
                        (-1, 0) => draw_rectangle(
                            (segment.cur.0 as f32 * CELL_SIZE) - CELL_GAP / 2.0,
                            (segment.cur.1 as f32 * CELL_SIZE) + CELL_GAP / 2.0,
                            CELL_SIZE,
                            CELL_SIZE - CELL_GAP,
                            GREEN,
                        ),
                        (0, 1) => draw_rectangle(
                            (segment.cur.0 as f32 * CELL_SIZE) + CELL_GAP / 2.0,
                            (segment.cur.1 as f32 * CELL_SIZE) + CELL_GAP / 2.0,
                            CELL_SIZE - CELL_GAP,
                            CELL_SIZE,
                            GREEN,
                        ),
                        (0, -1) => draw_rectangle(
                            (segment.cur.0 as f32 * CELL_SIZE) + CELL_GAP / 2.0,
                            (segment.cur.1 as f32 * CELL_SIZE) - CELL_GAP / 2.0,
                            CELL_SIZE - CELL_GAP,
                            CELL_SIZE,
                            GREEN,
                        ),
                        _ => {}
                    }
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
