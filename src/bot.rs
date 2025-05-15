use macroquad::prelude::*;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn generate_hamiltonian_cycle(width: i32, height: i32) -> Vec<(usize, usize)> {
    if width != height {
        panic!("Width and height must be equal");
    }

    if width % 2 != 0 || height % 2 != 0 {
        panic!("Width and height must be even");
    }

    let arena_size = (width * height) as usize;
    let tour_to_number = generate_maze_tour(width, height);
    get_cycle_positions(&tour_to_number, width, height, arena_size)
}

fn generate_maze_tour(width: i32, height: i32) -> Vec<usize> {
    let arena_size = (width * height) as usize;
    let mut tour_to_number = vec![0; arena_size];

    let maze_width = width / 2;
    let maze_height = height / 2;
    let maze_size = (maze_width * maze_height) as usize;

    let mut nodes = vec![
        MazeNode {
            visited: false,
            can_go_right: false,
            can_go_down: false,
        };
        maze_size
    ];

    generate_maze_paths(&mut nodes, (-1, -1), (0, 0), maze_width, maze_height);

    generate_tour_numbers(&nodes, &mut tour_to_number, width, arena_size);

    tour_to_number
}

// Recursively generate maze paths
fn generate_maze_paths(
    nodes: &mut [MazeNode],
    from: (i32, i32),
    cur: (i32, i32),
    maze_width: i32,
    maze_height: i32,
) {
    let (from_x, from_y) = from;
    let (x, y) = cur;

    if x < 0 || y < 0 || x >= maze_width || y >= maze_height {
        return;
    }
    if MazeNode::is_visited(nodes, cur, maze_width) {
        return;
    }
    MazeNode::mark_visited(nodes, cur, maze_width);

    if from_x != -1 {
        if from_x < x {
            MazeNode::mark_can_go_right(nodes, from, maze_width);
        } else if from_x > x {
            MazeNode::mark_can_go_right(nodes, cur, maze_width);
        } else if from_y < y {
            MazeNode::mark_can_go_down(nodes, from, maze_width);
        } else if from_y > y {
            MazeNode::mark_can_go_down(nodes, cur, maze_width);
        }
    }

    // Visit the four connected nodes randomly
    for _ in 0..2 {
        let r = rand::gen_range(0, 4);
        match r {
            0 => generate_maze_paths(nodes, cur, (x - 1, y), maze_width, maze_height),
            1 => generate_maze_paths(nodes, cur, (x + 1, y), maze_width, maze_height),
            2 => generate_maze_paths(nodes, cur, (x, y - 1), maze_width, maze_height),
            3 => generate_maze_paths(nodes, cur, (x, y + 1), maze_width, maze_height),
            _ => {}
        }
    }

    // Visit all remaining directions
    generate_maze_paths(nodes, cur, (x - 1, y), maze_width, maze_height);
    generate_maze_paths(nodes, cur, (x + 1, y), maze_width, maze_height);
    generate_maze_paths(nodes, cur, (x, y + 1), maze_width, maze_height);
    generate_maze_paths(nodes, cur, (x, y - 1), maze_width, maze_height);
}

// Generate the tour numbers that define the Hamiltonian cycle
fn generate_tour_numbers(
    nodes: &[MazeNode],
    tour_to_number: &mut [usize],
    arena_width: i32,
    arena_size: usize,
) {
    let maze_width = arena_width / 2;

    let (mut x, mut y) = (0, 0);
    let start_dir = if MazeNode::can_go_down(nodes, (x, y), maze_width) {
        Direction::Up
    } else {
        Direction::Left
    };
    let mut dir = start_dir;
    let mut number: usize = 0;

    loop {
        let next_dir = find_next_dir(nodes, (x, y), dir, maze_width);

        match dir {
            Direction::Right => {
                set_tour_number(tour_to_number, (x * 2, y * 2), number, arena_width);
                number += 1;
                if next_dir == dir || next_dir == Direction::Down || next_dir == Direction::Left {
                    set_tour_number(tour_to_number, (x * 2 + 1, y * 2), number, arena_width);
                    number += 1;
                }
                if next_dir == Direction::Down || next_dir == Direction::Left {
                    set_tour_number(tour_to_number, (x * 2 + 1, y * 2 + 1), number, arena_width);
                    number += 1;
                }
                if next_dir == Direction::Left {
                    set_tour_number(tour_to_number, (x * 2, y * 2 + 1), number, arena_width);
                    number += 1;
                }
            }
            Direction::Down => {
                set_tour_number(tour_to_number, (x * 2 + 1, y * 2), number, arena_width);
                number += 1;
                if next_dir == dir || next_dir == Direction::Left || next_dir == Direction::Up {
                    set_tour_number(tour_to_number, (x * 2 + 1, y * 2 + 1), number, arena_width);
                    number += 1;
                }
                if next_dir == Direction::Left || next_dir == Direction::Up {
                    set_tour_number(tour_to_number, (x * 2, y * 2 + 1), number, arena_width);
                    number += 1;
                }
                if next_dir == Direction::Up {
                    set_tour_number(tour_to_number, (x * 2, y * 2), number, arena_width);
                    number += 1;
                }
            }
            Direction::Left => {
                set_tour_number(tour_to_number, (x * 2 + 1, y * 2 + 1), number, arena_width);
                number += 1;
                if next_dir == dir || next_dir == Direction::Up || next_dir == Direction::Right {
                    set_tour_number(tour_to_number, (x * 2, y * 2 + 1), number, arena_width);
                    number += 1;
                }
                if next_dir == Direction::Up || next_dir == Direction::Right {
                    set_tour_number(tour_to_number, (x * 2, y * 2), number, arena_width);
                    number += 1;
                }
                if next_dir == Direction::Right {
                    set_tour_number(tour_to_number, (x * 2 + 1, y * 2), number, arena_width);
                    number += 1;
                }
            }
            Direction::Up => {
                set_tour_number(tour_to_number, (x * 2, y * 2 + 1), number, arena_width);
                number += 1;
                if next_dir == dir || next_dir == Direction::Right || next_dir == Direction::Down {
                    set_tour_number(tour_to_number, (x * 2, y * 2), number, arena_width);
                    number += 1;
                }
                if next_dir == Direction::Right || next_dir == Direction::Down {
                    set_tour_number(tour_to_number, (x * 2 + 1, y * 2), number, arena_width);
                    number += 1;
                }
                if next_dir == Direction::Down {
                    set_tour_number(tour_to_number, (x * 2 + 1, y * 2 + 1), number, arena_width);
                    number += 1;
                }
            }
        }

        dir = next_dir;

        match next_dir {
            Direction::Right => x += 1,
            Direction::Left => x -= 1,
            Direction::Down => y += 1,
            Direction::Up => y -= 1,
        }

        if number >= arena_size {
            break;
        }
    }
}

// Convert the tour numbers to a sequence of positions for rendering
fn get_cycle_positions(
    tour_to_number: &[usize],
    arena_width: i32,
    arena_height: i32,
    arena_size: usize,
) -> Vec<(usize, usize)> {
    let mut positions = vec![(0, 0); arena_size];

    // For each position in the arena, store its coordinates at its tour number index
    for x in 0..arena_width {
        for y in 0..arena_height {
            let tour_number = get_path_number(tour_to_number, (x, y), arena_width) as usize;
            if tour_number < arena_size {
                positions[tour_number] = (x as usize, y as usize);
            }
        }
    }

    positions
}

// Get the path number for a specific position
fn get_path_number(tour_to_number: &[usize], (x, y): (i32, i32), arena_width: i32) -> usize {
    tour_to_number[(x + arena_width * y) as usize]
}

fn find_next_dir(
    nodes: &[MazeNode],
    coord: (i32, i32),
    dir: Direction,
    maze_width: i32,
) -> Direction {
    match dir {
        Direction::Right => {
            if MazeNode::can_go_up(nodes, coord, maze_width) {
                return Direction::Up;
            }
            if MazeNode::can_go_right(nodes, coord, maze_width) {
                return Direction::Right;
            }
            if MazeNode::can_go_down(nodes, coord, maze_width) {
                return Direction::Down;
            }
            Direction::Left
        }
        Direction::Down => {
            if MazeNode::can_go_right(nodes, coord, maze_width) {
                return Direction::Right;
            }
            if MazeNode::can_go_down(nodes, coord, maze_width) {
                return Direction::Down;
            }
            if MazeNode::can_go_left(nodes, coord, maze_width) {
                return Direction::Left;
            }
            Direction::Up
        }
        Direction::Left => {
            if MazeNode::can_go_down(nodes, coord, maze_width) {
                return Direction::Down;
            }
            if MazeNode::can_go_left(nodes, coord, maze_width) {
                return Direction::Left;
            }
            if MazeNode::can_go_up(nodes, coord, maze_width) {
                return Direction::Up;
            }
            Direction::Right
        }
        Direction::Up => {
            if MazeNode::can_go_left(nodes, coord, maze_width) {
                return Direction::Left;
            }
            if MazeNode::can_go_up(nodes, coord, maze_width) {
                return Direction::Up;
            }
            if MazeNode::can_go_right(nodes, coord, maze_width) {
                return Direction::Right;
            }
            Direction::Down
        }
    }
}

// Set tour number if not already set
fn set_tour_number(
    tour_to_number: &mut [usize],
    (x, y): (i32, i32),
    number: usize,
    arena_width: i32,
) {
    let index = (x + arena_width * y) as usize;
    if index < tour_to_number.len() && tour_to_number[index] == 0 {
        tour_to_number[index] = number;
    }
}

#[derive(Clone)]
struct MazeNode {
    visited: bool,
    can_go_right: bool,
    can_go_down: bool,
}

impl MazeNode {
    fn mark_visited(nodes: &mut [MazeNode], (x, y): (i32, i32), maze_width: i32) {
        nodes[(x + y * maze_width) as usize].visited = true;
    }

    fn mark_can_go_right(nodes: &mut [MazeNode], (x, y): (i32, i32), maze_width: i32) {
        nodes[(x + y * maze_width) as usize].can_go_right = true;
    }

    fn mark_can_go_down(nodes: &mut [MazeNode], (x, y): (i32, i32), maze_width: i32) {
        nodes[(x + y * maze_width) as usize].can_go_down = true;
    }

    fn can_go_right(nodes: &[MazeNode], (x, y): (i32, i32), maze_width: i32) -> bool {
        nodes[(x + y * maze_width) as usize].can_go_right
    }

    fn can_go_down(nodes: &[MazeNode], (x, y): (i32, i32), maze_width: i32) -> bool {
        nodes[(x + y * maze_width) as usize].can_go_down
    }

    fn can_go_left(nodes: &[MazeNode], (x, y): (i32, i32), maze_width: i32) -> bool {
        if x == 0 {
            return false;
        }
        nodes[((x - 1) + y * maze_width) as usize].can_go_right
    }

    fn can_go_up(nodes: &[MazeNode], (x, y): (i32, i32), maze_width: i32) -> bool {
        if y == 0 {
            return false;
        }
        nodes[(x + (y - 1) * maze_width) as usize].can_go_down
    }

    fn is_visited(nodes: &[MazeNode], (x, y): (i32, i32), maze_width: i32) -> bool {
        nodes[(x + y * maze_width) as usize].visited
    }
}
