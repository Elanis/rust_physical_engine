mod physics;

extern crate minifb;

use minifb::{Key, WindowOptions, Window};

use physics::world::World;
use physics::cube::Cube;

use physics::vec3::Vec3;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
	let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

	let mut window = Window::new("Rust Physical Engine - ESC to exit",
								 WIDTH,
								 HEIGHT,
								 WindowOptions::default()).unwrap_or_else(|e| {
		panic!("{}", e);
	});

	// Initialize world
	let mut _world = World::new();

	_world.set_gravity(10.0);

	_world.add_entity(
		Box::new(
			Cube::new(
				Vec3::new((WIDTH/2 - 25) as f32, (HEIGHT/2 - 25) as f32, 0.0),
				0xFF0000,
				50
			)
		)
	);

	_world.add_entity(
		Box::new(
			Cube::new(
				Vec3::new((WIDTH/2 - 50) as f32, (HEIGHT/2 - 100) as f32, 0.0),
				0x0000FF,
				50
			)
		)
	);

	// Draw window
	while window.is_open() && !window.is_key_down(Key::Escape) {
		// Render background
		for i in buffer.iter_mut() {
			*i = 0xEEEEEE;
		}

		_world.simulate(&mut buffer, WIDTH, HEIGHT);

		// Really draw what we rendered
		window.update_with_buffer(&buffer).unwrap();
	}
}