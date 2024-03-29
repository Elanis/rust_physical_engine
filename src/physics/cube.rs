use super::entity::Entity;

use super::vec3::Vec3;

pub struct Cube {
	_position : Vec3,
	_angles: Vec3,

	_velocity: Vec3,
	_angle_velocity: Vec3,

	_immutable: bool,

	_width: u32,
	_color: u32,

	_bouncing_value: f32
}

impl Entity for Cube {
	fn get_position(&self) -> Vec3 {
		return self._position;
	}

	fn get_angles(&self) -> Vec3 {
		return self._angles;
	}

	fn get_velocity(&self) -> Vec3 {
		return self._velocity;
	}

	fn apply_velocity(&mut self, velocity: Vec3) {
		self._velocity = self._velocity + velocity
	}

	fn get_angle_velocity(&self) -> Vec3 {
		return self._angle_velocity;
	}

	fn render(&self, _buffer : &mut Vec<u32>, width : usize, height : usize) {
		for x in 0..self._width {
			for y in 0..self._width {
				let line : usize = (y + self._position.y as u32) as usize;

				if line >= height {
					continue;
				}

				_buffer[(line * width) + (x + self._position.x as u32) as usize] = self._color;
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

		self._angles.x += self._angle_velocity.z * tick;
		self._angles.y += self._angle_velocity.y * tick;
		self._angles.z += self._angle_velocity.z * tick;
	}

	fn is_immutable(&self) -> bool {
		self._immutable
	}

	fn get_up(&self) -> f32 {
		self._position.y
	}

	fn get_left(&self) -> f32 {
		self._position.x
	}

	fn get_bottom(&self) -> f32 {
		(self._position.y + self._width as f32)
	}

	fn get_right(&self) -> f32 {
		(self._position.x + self._width as f32)
	}

	fn get_radius(&self) -> f32 {
		(self._width as f32 / 2.0)
	}

	fn get_bouncing_value(&self) -> f32 {
		self._bouncing_value
	}

	fn is_member(&self, postion : &Vec3) -> bool {
		(postion.y < self.get_bottom() &&
			postion.y > self.get_up() &&
			postion.x > self.get_left() &&
			postion.x < self.get_right()
		)
	}
}

impl Cube {
	pub fn new(postion : Vec3, color : u32, width : u32) -> Cube {
		(Cube::create(postion, color, width, false))
	}

	pub fn new_immutable(postion : Vec3, color : u32, width : u32) -> Cube {
		(Cube::create(postion, color, width, true))
	}

	pub fn create(postion : Vec3, color : u32, width : u32, immutable : bool) -> Cube {
		return Cube {
			_position : postion,
			_angles: Vec3::new(0.0, 0.0, 0.0),

			_velocity: Vec3::new(0.0, 0.0, 0.0),
			_angle_velocity: Vec3::new(0.0, 0.0, 0.0),

			_immutable: immutable,

			_width: width,

			_color: color,

			_bouncing_value: 0.9, // TODO: custom value
		}
	}
}