extern crate rand;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

use rand::Rng;

const PIXEL_SIZE: u32 = 3;
const GRID_SPACING: u32 = 1;
const GRID_WIDTH: u32 = 350;
const GRID_HEIGHT: u32 = 175;

#[derive(Clone)]
struct Cell {
    alive: bool,
    neighbors: Vec<usize>,
}

impl Cell {
	pub fn new(alive: bool) -> Cell {
		Cell {
			alive,
			neighbors: Vec::new(),
		}
	}
}

fn is_outer(index: usize) -> bool {
	let _x = index as u32 % (GRID_WIDTH + 2);
	let _y = index as u32 / (GRID_WIDTH + 2);
	if (_x > 0) && (_x < GRID_WIDTH + 1) && (_y > 0) && (_y < GRID_HEIGHT + 1) {
		false
	}
	else {
		true
	}
}

fn get_neighbors(index: usize) -> Vec<usize> {
	let upper_left = index - GRID_WIDTH as usize - 2 - 1;
	let upper = index - (GRID_WIDTH as usize + 2);
	let upper_right = index - GRID_WIDTH as usize - 2 + 1;
	let left = index - 1;
	let right = index + 1;
	let lower_left = index + GRID_WIDTH as usize + 2 - 1;
	let lower = index + GRID_WIDTH as usize + 2;
	let lower_right = index + GRID_WIDTH as usize + 2 + 1;

	/*
	let _x = index as u32 % GRID_WIDTH;
	let _y = index as u32 / GRID_WIDTH;

	let neighbors = match (_x, _y) {
		(0, 0) => {
			vec![right, lower, lower_right]
		},
		(0, 1...STL_HEIGHT) => {
			vec![upper, upper_right, right, lower, lower_right]
		},
		(0, MAX_HEIGHT) => {
			vec![upper, upper_right, right]
		}
		(1...STL_WIDTH, 0) => {
			vec![left, right, lower_left, lower, lower_right]
		},
		(1...STL_WIDTH, 1...STL_HEIGHT) => {
			vec![upper_left, upper, upper_right, left, right, lower_left, lower, lower_right]
		},
		(1...STL_WIDTH, MAX_HEIGHT) => {
			vec![upper_left, upper, upper_right, left, right]
		},
		(MAX_WIDTH, 0) => {
			vec![left, lower_left, lower]
		},
		(MAX_WIDTH, 1...STL_HEIGHT) => {
			vec![upper_left, upper, left, lower_left, lower]
		},
		(MAX_WIDTH, MAX_HEIGHT) => {
			vec![upper_left, upper, left]
		},
		(_, _) => { vec![] }
	};
	neighbors*/
	vec![upper_left, upper, upper_right, left, right, lower_left, lower, lower_right]
}

fn evolve(grid: &mut Vec<Cell>) {
	let grid2 = grid.clone();
	for (_, cell) in grid.iter_mut().enumerate().filter(|c| !is_outer(c.0)) {
		let mut count = 0;
		for &n in cell.neighbors.iter() {
			let neighbor = grid2.get(n as usize).unwrap();
			if neighbor.alive {
				count += 1;
			}
		}

		if count > 3 || count < 2 {
			cell.alive = false;
		}
			else if count == 3 {
				cell.alive = true;
			}
	}
}

fn main() {
    // SDL setup
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

	let mut step = 60;
	let mut running = false;
	let mut clicking = false;
	let mut initial:Option<bool> = None;

    let window_width = GRID_WIDTH * PIXEL_SIZE + GRID_SPACING * GRID_WIDTH - 1;
    let window_height = GRID_HEIGHT * PIXEL_SIZE + GRID_SPACING * GRID_HEIGHT - 1;

    let window = video_subsystem
        .window("Conway's Game of Life", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = sdl_context.event_pump().unwrap();

    canvas.set_draw_color(Color::RGB(32, 32, 32));
    canvas.clear();
    canvas.present();

    // Game of Life setup
    let grid_size = (GRID_WIDTH as usize + 2) * (GRID_HEIGHT as usize + 2);
	let mut rng = rand::thread_rng();
    let mut grid: Vec<Cell> = vec![Cell::new(false); grid_size];
	for (i, cell) in grid.iter_mut().enumerate().filter(|c| !is_outer(c.0)) {
		//cell.alive = rng.gen_bool(0.5);

		cell.neighbors = get_neighbors(i);
	}

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,

                Event::KeyDown { keycode: Some(keycode), .. } => match keycode {
                    Keycode::Escape => break 'main,
					Keycode::R => {
						for (_, cell) in grid.iter_mut().enumerate().filter(|c| !is_outer(c.0)) {
							cell.alive = rng.gen_bool(0.5);
						}
					},
					Keycode::Up => {
						if step < 503_316_480u32 { // 60 × 2^something
							step *= 2;
						}
					},
					Keycode::Down => {
						if step > 1 {
							step /= 2;
						}
					},
					Keycode::Space => {
						running = !running;
					},
					Keycode::Period => {
						evolve(&mut grid);
					},
					Keycode::C | Keycode::Backspace => {
						for cell in grid.iter_mut() {
							cell.alive = false;
						}
					}
                    _ => {}
                },
				Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
					clicking = true;
				},
				Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {
					clicking = false;
					initial = None;
				},
				Event::MouseMotion { x, y, .. } => {
					if clicking {
						let x = x as u32 / (PIXEL_SIZE + GRID_SPACING) + 1;
						let y = y as u32 / (PIXEL_SIZE + GRID_SPACING) + 1;
						let index = y * (GRID_WIDTH + 2) + x;
						if initial == None {
							initial = match grid.get_mut(index as usize) {
								Some(cell) => { Some(!cell.alive) },
								None => { Some(true) }
							};
						}
						match grid.get_mut(index as usize) {
							Some(cell) => {
								cell.alive = initial.unwrap();
							},
							None => {}
						}
					}
					//println!("{}, {}", x, y);
				}

                _ => {}
            }
        }

		if running {
			canvas.set_draw_color(Color::RGB(32, 32, 32));
			canvas.clear();
			canvas.set_draw_color(Color::RGB(224, 224, 224));
		}
		else {
			canvas.set_draw_color(Color::RGB(64, 64, 64));
			canvas.clear();
			canvas.set_draw_color(Color::RGB(128, 128, 128));
		}

		for (i, cell) in grid.iter().enumerate().filter(|c| !is_outer(c.0) ) {
			let _index = i as u32 - GRID_WIDTH - 3;
			let _x = (_index % (GRID_WIDTH + 2) * PIXEL_SIZE) as i32;
			let _x = _x + _x / PIXEL_SIZE as i32 * GRID_SPACING as i32;
			let _y = (_index / (GRID_WIDTH + 2) * PIXEL_SIZE) as i32;
			let _y = _y + _y / PIXEL_SIZE as i32 * GRID_SPACING as i32;
			//println!("({}, {})", _x, _y);
			if cell.alive {
				canvas.fill_rect(Rect::new(_x, _y, PIXEL_SIZE, PIXEL_SIZE)).unwrap();
			}
		}

		if running {
			evolve(&mut grid);
		}

		canvas.present();
		if running {
			std::thread::sleep(Duration::new(0, 1_000_000_000u32 / step));
		}
		else {
			std::thread::sleep(Duration::new(0, 1));
		}
    }
}
