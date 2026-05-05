use alloc::collections::VecDeque;
use crate::vga_buffer::{write_pixel, Color};
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::instructions::interrupts;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction { Up, Down, Left, Right }

#[derive(PartialEq, Copy, Clone)]
pub struct Point { pub x: usize, pub y: usize }

pub struct SnakeGame {
    pub body: VecDeque<Point>,
    pub direction: Direction,
    pub food: Point,
    pub score: u32,
    pub alive: bool,
}

lazy_static! {
    pub static ref GAME: Mutex<SnakeGame> = Mutex::new(SnakeGame::new());
}

impl SnakeGame {
    fn new() -> Self {
        let mut body = VecDeque::new();
        body.push_front(Point { x: 40, y: 12 });
        Self {
            body,
            direction: Direction::Right,
            food: Point { x: 10, y: 10 },
            score: 0,
            alive: true,
        }
    }
}

pub fn start_game() {
    draw_border();

    loop {
        for _ in 0..150_000_000usize {
            unsafe { core::arch::asm!("nop") }
        }

        let mut keep_going = true;

        interrupts::without_interrupts(|| {
            let mut game = GAME.lock();

            if !game.alive {
                keep_going = false;
                return;
            }

            let head = *game.body.front().unwrap();

            let new_head = match game.direction {
                Direction::Up    => Point { x: head.x, y: head.y.saturating_sub(1) },
                Direction::Down  => Point { x: head.x, y: (head.y + 1).min(24) },
                Direction::Left  => Point { x: head.x.saturating_sub(1), y: head.y },
                Direction::Right => Point { x: (head.x + 1).min(79), y: head.y },
            };

            if new_head.x == 0 || new_head.x == 79 || new_head.y == 0 || new_head.y == 24 {
                game.alive = false;
                drop(game);
                handle_game_over();
                keep_going = false;
                return;
            }

            // self collision
            if game.body.contains(&new_head) {
                game.alive = false;
                drop(game);
                handle_game_over();
                keep_going = false;
                return;
            }

            game.body.push_front(new_head);
            let food = game.food;
            let score = game.score;

            if new_head == food {
                game.score += 1;
                game.food = Point {
                    x: (score as usize * 7 + 13) % 78 + 1,
                    y: (score as usize * 3 + 7)  % 23 + 1,
                };
                let new_food = game.food;
                drop(game);
                write_pixel(new_head.x, new_head.y, 0xDB, Color::Green);
                write_pixel(new_food.x, new_food.y, 0xA2, Color::Red);
            } else {
                let tail = game.body.pop_back().unwrap();
                drop(game);
                write_pixel(new_head.x, new_head.y, 0xDB, Color::Green);
                write_pixel(tail.x, tail.y, b' ', Color::Black);
            }
        });

        if !keep_going {
            break;
        }
    }

    loop { x86_64::instructions::hlt(); }
}

fn draw_border() {
    for x in 0..80 {
        write_pixel(x, 0,  0xC4, Color::White);
        write_pixel(x, 24, 0xC4, Color::White);
    }
    for y in 0..25 {
        write_pixel(0,  y, 0xB3, Color::White);
        write_pixel(79, y, 0xB3, Color::White);
    }
    write_pixel(10, 10, 0xA2, Color::Red);
}

fn handle_game_over() {
    let msg = b"GAME OVER";
    for (i, &byte) in msg.iter().enumerate() {
        write_pixel(35 + i, 12, byte, Color::Red);
    }
}