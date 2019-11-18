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

	_world.set_gravity(-1.0);
	_world.add_entity(
		Box::new(
			Cube::new(
				10,
				10,
				Vec3::new(400.0, 400.0, 0.0)
			)
		)
	);

	let mut initialized = false;

	// Draw window
	while window.is_open() && !window.is_key_down(Key::Escape) {
		if !initialized {
			for i in buffer.iter_mut() {
				*i = 0xFFFFFF; // write something more funny here!
			}

			initialized = true;
		} else {
			_world.simulate(&buffer);
		}

		// We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
		window.update_with_buffer(&buffer).unwrap();
	}
}