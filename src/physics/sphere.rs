use super::entity::Entity;

use super::vec3::Vec3;

pub struct Sphere {
	_position : Vec3,

	_velocity: Vec3,

	_immutable: bool,

	_radius: u32,
	_color: u32,

	_bouncing_value: f32
}

impl Entity for Sphere {
	fn get_position(&self) -> Vec3 {
		self._position
	}

	fn get_angles(&self) -> Vec3 {
		Vec3::new(0.0,0.0,0.0) // Spheres does not have angles
	}

	fn get_velocity(&self) -> Vec3 {
		return self._velocity;
	}

	fn apply_velocity(&mut self, velocity: Vec3) {
		self._velocity = self._velocity + velocity
	}

	fn get_angle_velocity(&self) -> Vec3 {
		Vec3::new(0.0,0.0,0.0) // Spheres does not have angles
	}

	fn render(&self, _buffer : &mut Vec<u32>, width : usize, height : usize) {
		for x in 0..(self._radius*2) {
			for y in 0..(self._radius*2) {
				let real_x : usize = (self._position.x as i32 + x as i32 - self._radius as i32) as usize;
				let real_y : usize = (self._position.y as i32 + y as i32 - self._radius as i32) as usize;

				if real_y >= height || !self.is_member(&Vec3::new(real_x as f32, real_y as f32, 0.0)) {
					continue;
				}

				_buffer[(real_y * width) + real_x] = self._color;
			}
		}

		// TODO: 3D Render
	}

	fn recalc_speed(&mut self, tick : f32, gravity : f32) {
		if self._immutable { return; }
		
		self._velocity.y += gravity * tick;
	}

	fn recalc_pos(&mut self, tick : f32) {
		self._position.x += self._velocity.z * tick;
		self._position.y += self._velocity.y * tick;
		self._position.z += self._velocity.z * tick;
	}

	fn is_immutable(&self) -> bool {
		self._immutable
	}

	fn get_up(&self) -> f32 {
		0.0
	}

	fn get_left(&self) -> f32 {
		0.0
	}

	fn get_bottom(&self) -> f32 {
		0.0
	}

	fn get_right(&self) -> f32 {
		0.0
	}

	fn get_radius(&self) -> f32 {
		(self._radius as f32)
	}

	fn get_bouncing_value(&self) -> f32 {
		self._bouncing_value
	}

	fn is_member(&self, position : &Vec3) -> bool {
		(self._position.distance(&position) <= self._radius as f32)
	}
}

impl Sphere {
	pub fn new(position : Vec3, color : u32, radius : u32) -> Sphere {
		(Sphere::create(position, color, radius, false))
	}

	pub fn new_immutable(position : Vec3, color : u32, radius : u32) -> Sphere {
		(Sphere::create(position, color, radius, true))
	}

	pub fn create(position : Vec3, color : u32, radius : u32, immutable : bool) -> Sphere {
		return Sphere {
			_position : position,

			_velocity: Vec3::new(0.0, 0.0, 0.0),

			_immutable: immutable,

			_radius: radius,

			_color: color,

			_bouncing_value: 0.9, // TODO: custom value
		}
	}
}