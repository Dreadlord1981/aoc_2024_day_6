use std::{collections::HashMap, fs};
use std::time::Instant;

#[derive(Debug, Default, Clone)]
struct Map {
	width: usize,
	height: usize,
	rows: Vec<String>,
	walls: HashMap<(usize, usize), i32>
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
	#[default]
	UP,
	DOWN,
	LEFT,
	RIGHT
}

#[derive(Debug, Default, Clone, Copy)]
struct Guard {
	x: i32,
	y: i32,
	direction: Direction
}

fn main() {

	let input = fs::read_to_string("floor.txt").unwrap();
	let (mut map, mut guard) = init(&input).unwrap();

	let guard_clone = guard.clone();
	let map_clone = map.clone();

	let visited = find_path(&mut map, &mut guard);

	println!("Part 1: {}", visited.len());

	let now = Instant::now();

	let mut count = 0;

	for (x, y) in visited.iter() {


		let mut guard = guard_clone.clone();
		let mut map = map_clone.clone();

		map.walls.insert((*x as usize, *y as usize), 0);
		
		let is_loop = is_loop(&mut map, &mut guard);

		if is_loop {
			count += 1;
		}
	}

	let elapsed = now.elapsed();
	
	println!("Part 2: {count}");
    println!("Elapsed Part 2: {:.2?}", elapsed);

}

fn init(input: &str) -> Result<(Map, Guard), ()> {

	let mut guard = Guard::default();
	let mut walls: HashMap::<(usize, usize), i32> = HashMap::new();

	let data: Vec<String> = input
	.lines()
	.map(|line| {
		line.to_string()
	})
	.collect();

	for (y, line) in data.iter().enumerate() {

		for (x, char) in line.chars().enumerate() {
			
			if char == '^' || char == 'v' || char == '<' || char == '>' {

				guard = Guard {
					x: x as i32,
					y: y as i32,
					direction: match char {
						'v' =>  {
							Direction::DOWN
						},
						'^' =>  {
							Direction::UP
						},
						'>' =>  {
							Direction::RIGHT
						},
						'<' =>  {
							Direction::LEFT
						},
						_ => {
							Direction::UP
						}
					}
				};
			}
			else if char == '#' {
				walls.insert((x as usize, y as usize), 0);
			}
		}
	}

	let width = data.get(0).unwrap().len() - 1;
	let height = data.len() - 1;

	Ok((Map {
		rows: data,
		walls,
		width: width,
		height: height
	}, guard))
}

fn find_path(map: &mut Map, guard: &mut Guard) -> Vec<(i32, i32)> {

	let width = map.width;
	let height = map.height;

	let mut steps = vec![( guard.x,  guard.y)];

	loop {

		let mut next_y = guard.y;
		let mut next_x = guard.x;

		match guard.direction {
			Direction::UP => {
				next_y -= 1; 
			},
			Direction::DOWN => {
				next_y  += 1; 
			},
			Direction::LEFT => {
				next_x -= 1; 
			},
			Direction::RIGHT => {
				next_x += 1; 
			}
		}

		if (next_y < 0 || next_y as usize > height) || (next_x < 0 || next_x as usize > width) {
			break;
		}

		let row = map.rows.get(next_y as usize).unwrap();

		if  !map.walls.contains_key(&(next_x as usize, next_y as usize)) {
			guard.y = next_y;
			guard.x = next_x;

			if !steps.contains(&(next_x, next_y)) {

				steps.push((next_x, next_y));

				let row = &row.chars().enumerate().map(|(i, c)| {

					let r = if i == next_x as usize {
						
						match guard.direction {
							Direction::UP => {
								'|'
							},
							Direction::DOWN => {
								'|'
							},
							Direction::LEFT => {
								'-'
							},
							Direction::RIGHT => {
								'-'
							}
						}
					}
					else {
						c
					};
	
					r
				})
				.collect::<String>();
	
				map.rows[next_y as usize] = row.to_string();
			}
		}
		else {

			let row = map.rows.get(guard.y as usize).unwrap();

			guard.direction = match guard.direction {
				Direction::UP => {
					Direction::RIGHT
				},
				Direction::DOWN => {
					Direction::LEFT
				},
				Direction::LEFT => {
					Direction::UP
				},
				Direction::RIGHT => {
					Direction::DOWN
				}
			};

			let row = &row.chars().enumerate().map(|(i, c)| {

				let r = if i == guard.x as usize {
					
					match guard.direction {
						Direction::UP => {
							'+'
						},
						Direction::DOWN => {
							'+'
						},
						Direction::LEFT => {
							'+'
						},
						Direction::RIGHT => {
							'+'
						}
					}
				}
				else {
					c
				};

				r
			})
			.collect::<String>();

			map.rows[guard.y as usize] = row.to_string();
		}

	}

	steps
}

fn is_loop(map: &mut Map, guard: &mut Guard) -> bool {

	let width = map.width;
	let height = map.height;

	let mut visited = HashMap::<(i32, i32, Direction), i32>::new();
	let mut is_loop = false;

	loop {

		let mut next_y = guard.y;
		let mut next_x = guard.x;

		match guard.direction {
			Direction::UP => {
				next_y -= 1; 
			},
			Direction::DOWN => {
				next_y  += 1; 
			},
			Direction::LEFT => {
				next_x -= 1; 
			},
			Direction::RIGHT => {
				next_x += 1; 
			}
		}

		if (next_y < 0 || next_y as usize > height) || (next_x < 0 || next_x as usize > width) {
			break;
		}

		if  !map.walls.contains_key(&(next_x as usize, next_y as usize)) {
			guard.y = next_y;
			guard.x = next_x;			
		}
		else {

			if !visited.contains_key(&(guard.x, guard.y, guard.direction)) {
				visited.insert((guard.x, guard.y, guard.direction), 1);
			}
			else {
				is_loop = true;
				break;
			}

			guard.direction = match guard.direction {
				Direction::UP => {
					Direction::RIGHT
				},
				Direction::DOWN => {
					Direction::LEFT
				},
				Direction::LEFT => {
					Direction::UP
				},
				Direction::RIGHT => {
					Direction::DOWN
				}
			};
		}

	}

	is_loop
}