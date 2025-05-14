use macroquad::prelude::*;

pub const SNAKE_SPEED: f32 = 15.;
pub const CELL_SIZE: f32 = 10.;
pub const CELL_GAP: f32 = 2.5;

pub struct Snake {
    pub segments: Vec<SnakeSegment>,
    cur_directions: (i32, i32),
    future_directions: (i32, i32),
}

pub struct SnakeSegment {
    pub cur: (usize, usize),
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
        let ratio = (snake_timer * SNAKE_SPEED).min(1.0);
        let mut last_was_corner = false;

        for (i, segment) in self.segments.iter().enumerate() {
            // Calculate the interpolated position and movement direction
            let (draw_x, draw_y, direction) = self.calculate_segment_position(i, ratio);

            // Draw the segment based on whether the last segment was a corner
            if last_was_corner {
                self.draw_corner_piece(segment, direction, ratio);
            } else {
                self.draw_segment(draw_x, draw_y, direction);
            }

            // Check if this segment forms a corner piece
            last_was_corner = i > 0 && i < self.segments.len() - 1 && self.is_corner_piece(i);

            // Draw the current segment as a corner piece if needed
            if last_was_corner {
                let x = segment.cur.0 as f32 * CELL_SIZE;
                let y = segment.cur.1 as f32 * CELL_SIZE;
                self.draw_segment(x, y, direction);
            }
        }
    }

    pub fn is_eating(&self, apple: (usize, usize)) -> bool {
        let head = &self.segments[0];
        head.cur == apple
    }

    pub fn grow(&mut self) {
        let last_segment = &self.segments[self.segments.len() - 1];
        let new_segment = SnakeSegment {
            cur: last_segment.cur,
            prev: last_segment.cur,
        };
        self.segments.push(new_segment);
    }

    fn calculate_segment_position(&self, index: usize, ratio: f32) -> (f32, f32, (i32, i32)) {
        let segment = &self.segments[index];

        if index == 0 {
            // Head segment: interpolate between prev and cur positions
            let prev = segment.prev;
            let cur = segment.cur;
            let x = (prev.0 as f32 * (1.0 - ratio) + cur.0 as f32 * ratio) * CELL_SIZE;
            let y = (prev.1 as f32 * (1.0 - ratio) + cur.1 as f32 * ratio) * CELL_SIZE;
            let dir = (cur.0 as i32 - prev.0 as i32, cur.1 as i32 - prev.1 as i32);

            (x, y, dir)
        } else {
            // Body segments: interpolate toward the position of the segment ahead
            let future_pos = self.segments[index - 1].cur;
            let x =
                (segment.cur.0 as f32 * (1.0 - ratio) + future_pos.0 as f32 * ratio) * CELL_SIZE;
            let y =
                (segment.cur.1 as f32 * (1.0 - ratio) + future_pos.1 as f32 * ratio) * CELL_SIZE;
            let dir = (
                future_pos.0 as i32 - segment.cur.0 as i32,
                future_pos.1 as i32 - segment.cur.1 as i32,
            );

            (x, y, dir)
        }
    }

    fn is_corner_piece(&self, index: usize) -> bool {
        let prev_segment = &self.segments[index - 1];
        let next_segment = &self.segments[index + 1];

        // A corner is formed when previous and next segments have different directions
        prev_segment.cur.0 != next_segment.cur.0 && prev_segment.cur.1 != next_segment.cur.1
    }

    fn draw_corner_piece(&self, segment: &SnakeSegment, direction: (i32, i32), ratio: f32) {
        let (offset_x, offset_y, width, height) = match direction {
            (1, 0) => (
                CELL_GAP / 2. + (CELL_SIZE * ratio),
                CELL_GAP / 2.0,
                CELL_SIZE * 2. - CELL_GAP - (CELL_SIZE * ratio),
                CELL_SIZE - CELL_GAP,
            ),
            (-1, 0) => (
                CELL_GAP / 2. - CELL_SIZE,
                CELL_GAP / 2.0,
                CELL_SIZE * 2. - CELL_GAP - (CELL_SIZE * ratio),
                CELL_SIZE - CELL_GAP,
            ),
            (0, 1) => (
                CELL_GAP / 2.0,
                CELL_GAP / 2. + (CELL_SIZE * ratio),
                CELL_SIZE - CELL_GAP,
                CELL_SIZE * 2. - CELL_GAP - (CELL_SIZE * ratio),
            ),
            (0, -1) => (
                CELL_GAP / 2.0,
                CELL_GAP / 2. - CELL_SIZE,
                CELL_SIZE - CELL_GAP,
                CELL_SIZE * 2. - CELL_GAP - (CELL_SIZE * ratio),
            ),
            _ => return,
        };

        draw_rectangle(
            segment.cur.0 as f32 * CELL_SIZE + offset_x,
            segment.cur.1 as f32 * CELL_SIZE + offset_y,
            width,
            height,
            GREEN,
        );
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

    pub fn handle_input(&mut self) {
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

    pub fn step(&mut self) -> ((usize, usize), (usize, usize)) {
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

        // Return the new head and tail positions
        let head_pos = self.segments[0].cur;
        let tail_pos = self.segments[self.segments.len() - 1].prev;
        (head_pos, tail_pos)
    }

    pub fn is_dead(&self) -> bool {
        let map_width = screen_width() as usize / CELL_SIZE as usize;
        let map_height = screen_height() as usize / CELL_SIZE as usize;
        let head = &self.segments[0];

        // Check if the head is out of bounds
        if head.cur.0 >= map_width || head.cur.1 >= map_height {
            return true;
        }

        // Check if the head collides with its own body
        for segment in &self.segments[1..] {
            if head.cur == segment.cur {
                return true;
            }
        }

        false
    }
}
