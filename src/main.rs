mod physics;

extern crate minifb;

use minifb::{Key, WindowOptions, Window};

use physics::world::World;
use physics::cube::Cube;
use physics::sphere::Sphere;

use physics::vec3::Vec3;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
	let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

	let title = "Rust Physical Engine - ESC to exit - ";

	let mut window = Window::new("",
								 WIDTH,
								 HEIGHT,
								 WindowOptions::default()).unwrap_or_else(|e| {
		panic!("{}", e);
	});

	// Initialize world
	let mut _world = World::new();

	_world.set_gravity(150.0);

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

	_world.add_entity(
		Box::new(
			Cube::new_immutable(
				Vec3::new((WIDTH/2 - 25) as f32, (3*HEIGHT/4 - 25) as f32, 0.0),
				0x00FF00,
				50
			)
		)
	);

	_world.add_entity(
		Box::new(
			Sphere::new(
				Vec3::new((WIDTH/3 - 25) as f32, (HEIGHT/2 - 25) as f32, 0.0),
				0xFF0000,
				25
			)
		)
	);

	_world.add_entity(
		Box::new(
			Sphere::new(
				Vec3::new((WIDTH/3 - 50) as f32, (HEIGHT/2 - 100) as f32, 0.0),
				0x0000FF,
				25
			)
		)
	);

	_world.add_entity(
		Box::new(
			Sphere::new_immutable(
				Vec3::new((WIDTH/3 - 25) as f32, (3*HEIGHT/4 - 25) as f32, 0.0),
				0x00FF00,
				25
			)
		)
	);

	// Draw window
	while window.is_open() && !window.is_key_down(Key::Escape) {
		// Render background
		for i in buffer.iter_mut() {
			*i = 0xEEEEEE;
		}

		let fps = _world.simulate(&mut buffer, WIDTH, HEIGHT);

		window.set_title(&(title.to_owned() + &fps.to_string() + " FPS"));

		// Really draw what we rendered
		window.update_with_buffer(&buffer).unwrap();
	}
}