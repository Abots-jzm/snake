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
        let segments = (0..length)
            .map(|i| SnakeSegment {
                cur: (x - i, y),
                prev: (x - i, y),
            })
            .collect();

        Self {
            segments,
            cur_directions: (1, 0), // Start moving to the right
            future_directions: (1, 0),
        }
    }

    pub fn draw(&self, snake_timer: f32) {
        let ratio = snake_timer * SNAKE_SPEED;

        self.segments.iter().enumerate().for_each(|(i, segment)| {
            let (draw_location_x, draw_location_y, move_direction) = if i == 0 {
                // Head segment
                (
                    (segment.cur.0 as f32 + self.cur_directions.0 as f32 * ratio) * CELL_SIZE,
                    (segment.cur.1 as f32 + self.cur_directions.1 as f32 * ratio) * CELL_SIZE,
                    self.cur_directions,
                )
            } else {
                // Body segments
                let future_pos = self.segments[i - 1].cur;
                (
                    (segment.cur.0 as f32 * (1.0 - ratio) + future_pos.0 as f32 * ratio)
                        * CELL_SIZE,
                    (segment.cur.1 as f32 * (1.0 - ratio) + future_pos.1 as f32 * ratio)
                        * CELL_SIZE,
                    (
                        future_pos.0 as i32 - segment.cur.0 as i32,
                        future_pos.1 as i32 - segment.cur.1 as i32,
                    ),
                )
            };

            self.draw_segment(draw_location_x, draw_location_y, move_direction);

            // Add corner pieces for turns to fill gaps
            if i > 0 && i < self.segments.len() - 1 {
                self.draw_corner_if_needed(i, segment, move_direction);
            }
        });
    }

    fn draw_segment(&self, x: f32, y: f32, direction: (i32, i32)) {
        let (offset_x, offset_y, width, height) = match direction {
            (1, 0) => (
                CELL_GAP / 2.0,
                CELL_GAP / 2.0,
                CELL_SIZE,
                CELL_SIZE - CELL_GAP,
            ),
            (-1, 0) => (
                -CELL_GAP / 2.0,
                CELL_GAP / 2.0,
                CELL_SIZE,
                CELL_SIZE - CELL_GAP,
            ),
            (0, 1) => (
                CELL_GAP / 2.0,
                CELL_GAP / 2.0,
                CELL_SIZE - CELL_GAP,
                CELL_SIZE,
            ),
            (0, -1) => (
                CELL_GAP / 2.0,
                -CELL_GAP / 2.0,
                CELL_SIZE - CELL_GAP,
                CELL_SIZE,
            ),
            _ => return,
        };

        draw_rectangle(x + offset_x, y + offset_y, width, height, GREEN);
    }

    fn draw_corner_if_needed(&self, i: usize, segment: &SnakeSegment, move_direction: (i32, i32)) {
        let prev_segment = &self.segments[i - 1];
        let next_segment = &self.segments[i + 1];

        // A corner is formed when previous and next segments have different directions
        let is_corner =
            prev_segment.cur.0 != next_segment.cur.0 && prev_segment.cur.1 != next_segment.cur.1;

        if is_corner {
            let x = segment.cur.0 as f32 * CELL_SIZE;
            let y = segment.cur.1 as f32 * CELL_SIZE;
            self.draw_segment(x, y, move_direction);
        }
    }

    pub fn listen_for_input(&mut self) {
        let new_direction = if is_key_down(KeyCode::Up) {
            (0, -1)
        } else if is_key_down(KeyCode::Down) {
            (0, 1)
        } else if is_key_down(KeyCode::Left) {
            (-1, 0)
        } else if is_key_down(KeyCode::Right) {
            (1, 0)
        } else {
            return;
        };

        self.future_directions = new_direction;
    }

    pub fn step(&mut self) {
        // Update direction if not opposite
        if self.cur_directions.0 + self.future_directions.0 != 0
            || self.cur_directions.1 + self.future_directions.1 != 0
        {
            self.cur_directions = self.future_directions;
        }

        // Save current positions before moving
        for segment in &mut self.segments {
            segment.prev = segment.cur;
        }

        // Move the head
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
