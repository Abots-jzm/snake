use macroquad::prelude::*;

pub const SNAKE_SPEED: f32 = 500.;
pub const CELL_SIZE: f32 = 10.;
pub const CELL_GAP: f32 = 2.5;
const GROWTH_BUFFER_FOR_SHORTCUT: usize = 3;

// Helper function to get position index in cycle
fn get_position_in_cycle(pos: (usize, usize), cycle: &[(usize, usize)]) -> Option<usize> {
    cycle.iter().position(|&p| p == pos)
}

// Helper function to calculate forward distance on cycle
fn distance_on_cycle(from_idx: usize, to_idx: usize, cycle_len: usize) -> usize {
    if to_idx >= from_idx {
        to_idx - from_idx
    } else {
        cycle_len - from_idx + to_idx
    }
}

pub struct Snake {
    pub segments: Vec<SnakeSegment>,
    direction: (i32, i32),
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
            direction: (1, 0), // Start moving to the right
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

    fn get_next_direction(
        &self,
        cycle: &Vec<(usize, usize)>,
        apple_pos: (usize, usize),
    ) -> (i32, i32) {
        let map_width = screen_width() as usize / CELL_SIZE as usize;
        let map_height = screen_height() as usize / CELL_SIZE as usize;
        let head_pos = self.segments[0].cur;
        let current_snake_direction = self.direction;

        // Helper for collision check. Returns true if collision, false otherwise.
        let check_collision = |pos_to_check: (usize, usize), proposed_dir: (i32, i32)| -> bool {
            // Boundary check
            if pos_to_check.0 >= map_width || pos_to_check.1 >= map_height {
                return true;
            }
            // Self-collision check (with body S1..Sn-1)
            for i in 1..self.segments.len() {
                if pos_to_check == self.segments[i].cur {
                    return true;
                }
            }
            // Prevent moving directly backward if snake length > 1
            if self.segments.len() > 1 && current_snake_direction != (0, 0) {
                // current_snake_direction could be (0,0) if snake hasn't moved
                if proposed_dir.0 == -current_snake_direction.0
                    && proposed_dir.1 == -current_snake_direction.1
                {
                    return true; // Collision: trying to reverse
                }
            }
            false // No collision
        };

        let head_on_cycle_opt = get_position_in_cycle(head_pos, cycle);
        let food_on_cycle_opt = get_position_in_cycle(apple_pos, cycle);
        let tail_pos = self.segments.last().expect("Snake must have segments").cur;
        let tail_on_cycle_opt = get_position_in_cycle(tail_pos, cycle);

        // Try article's AI logic if head, food, and tail are on the cycle and cycle exists
        if !cycle.is_empty()
            && head_on_cycle_opt.is_some()
            && food_on_cycle_opt.is_some()
            && tail_on_cycle_opt.is_some()
        {
            let head_path_number = head_on_cycle_opt.unwrap();
            let food_path_number = food_on_cycle_opt.unwrap();
            let tail_path_number = tail_on_cycle_opt.unwrap();
            let cycle_len = cycle.len();

            let distance_to_food = distance_on_cycle(head_path_number, food_path_number, cycle_len);
            let distance_to_tail = distance_on_cycle(head_path_number, tail_path_number, cycle_len);

            let food_value = 1; // Assumed growth from one food item
            let snake_growth_length = 0; // Placeholder for accumulated future growth from article's logic

            let mut cutting_amount_available =
                if distance_to_tail > snake_growth_length + GROWTH_BUFFER_FOR_SHORTCUT {
                    distance_to_tail - snake_growth_length - GROWTH_BUFFER_FOR_SHORTCUT
                } else {
                    0
                };

            let arena_size = map_width * map_height;
            let snake_drawn_length = self.segments.len();
            let num_empty_squares_on_board = arena_size
                .saturating_sub(snake_drawn_length)
                .saturating_sub(snake_growth_length)
                .saturating_sub(food_value);

            if num_empty_squares_on_board < arena_size / 2 {
                cutting_amount_available = 0;
            } else if distance_to_food < distance_to_tail {
                // Food is between head and tail on cycle
                cutting_amount_available = cutting_amount_available.saturating_sub(food_value);
                if (distance_to_tail.saturating_sub(distance_to_food)) * 4
                    > num_empty_squares_on_board
                {
                    let future_food_penalty = 10; // Magic number from article
                    cutting_amount_available =
                        cutting_amount_available.saturating_sub(future_food_penalty);
                }
            }

            let cutting_amount_desired = distance_to_food;
            if cutting_amount_desired < cutting_amount_available {
                cutting_amount_available = cutting_amount_desired;
            }
            // Ensure cutting_amount_available is not negative (already handled by saturating_sub or initial check)
            // cutting_amount_available is now the max "length" of shortcut on cycle (number of cycle steps).

            let mut best_dir_candidate: Option<(i32, i32)> = None;
            let mut best_dist_cut = -1isize; // Maximize this value (length of shortcut on cycle)

            // Order of evaluation for shortcuts (Right, Left, Down, Up)
            let shortcut_eval_order = [
                (1, 0),  // Right
                (-1, 0), // Left
                (0, 1),  // Down
                (0, -1), // Up
            ];

            for &dir_candidate in &shortcut_eval_order {
                let next_potential_x = head_pos.0 as i32 + dir_candidate.0;
                let next_potential_y = head_pos.1 as i32 + dir_candidate.1;

                if next_potential_x < 0 || next_potential_y < 0 {
                    continue;
                }
                let next_potential_pos = (next_potential_x as usize, next_potential_y as usize);

                if !check_collision(next_potential_pos, dir_candidate) {
                    if let Some(next_pos_path_number) =
                        get_position_in_cycle(next_potential_pos, cycle)
                    {
                        let dist_on_cycle_to_next =
                            distance_on_cycle(head_path_number, next_pos_path_number, cycle_len)
                                as isize;

                        if dist_on_cycle_to_next <= cutting_amount_available as isize
                            && dist_on_cycle_to_next > best_dist_cut
                        {
                            best_dist_cut = dist_on_cycle_to_next;
                            best_dir_candidate = Some(dir_candidate);
                        }
                    }
                }
            }

            if let Some(dir) = best_dir_candidate {
                return dir;
            }
        }

        // Fallback Logic:
        // 1. Try default Hamiltonian cycle move (if head is on cycle and cycle exists)
        if !cycle.is_empty() && head_on_cycle_opt.is_some() {
            let head_idx = head_on_cycle_opt.unwrap();
            // cycle.len() > 0 is guaranteed by !cycle.is_empty()
            let next_target_idx = (head_idx + 1) % cycle.len();
            let next_target_pos = cycle[next_target_idx];
            let default_cycle_dir = (
                next_target_pos.0 as i32 - head_pos.0 as i32,
                next_target_pos.1 as i32 - head_pos.1 as i32,
            );
            if !check_collision(next_target_pos, default_cycle_dir) {
                return default_cycle_dir;
            }
        }

        // 2. Article's ordered fallback moves (Up, Left, Down, Right)
        let fallback_moves_ordered = [
            (0, -1), // Up
            (-1, 0), // Left
            (0, 1),  // Down
            (1, 0),  // Right
        ];
        for &fallback_dir in &fallback_moves_ordered {
            let next_potential_x = head_pos.0 as i32 + fallback_dir.0;
            let next_potential_y = head_pos.1 as i32 + fallback_dir.1;

            if next_potential_x < 0 || next_potential_y < 0 {
                continue;
            }
            let next_potential_pos = (next_potential_x as usize, next_potential_y as usize);
            if !check_collision(next_potential_pos, fallback_dir) {
                return fallback_dir;
            }
        }

        // 3. Absolute final fallback (snake is likely trapped)
        // Return the last direction from fallback_moves_ordered (Right), as per article.
        return fallback_moves_ordered.last().cloned().unwrap_or((1, 0)); // Default to Right
    }

    pub fn step(
        &mut self,
        cycle: &Vec<(usize, usize)>,
        apple_pos: (usize, usize),
    ) -> ((usize, usize), (usize, usize)) {
        let future_direction = self.get_next_direction(cycle, apple_pos);
        self.direction = future_direction;

        // Save current positions before moving
        for segment in &mut self.segments {
            segment.prev = segment.cur;
        }

        // Move the head
        let head = &mut self.segments[0];
        head.cur = (
            (head.cur.0 as i32 + self.direction.0) as usize,
            (head.cur.1 as i32 + self.direction.1) as usize,
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
