use super::positionnable::Positionnable;
use super::entity::Entity;

use super::vec3::Vec3;

pub struct Cube {
	_position : Vec3,
	_angles: Vec3,

	_velocity: Vec3,
	_angle_velocity: Vec3,

	_width: u32,
	_height: u32
}

impl Positionnable for Cube {
	fn get_position(&self) -> Vec3 {
		return self._position;
	}

	fn get_angles(&self) -> Vec3 {
		return self._angles;
	}
}

impl Entity for Cube {
	fn get_velocity(&self) -> Vec3 {
		return self._velocity;
	}

	fn get_angle_velocity(&self) -> Vec3 {
		return self._angle_velocity;
	}
}

impl Cube {
	pub fn new(width : u32, height : u32, postion : Vec3) -> Cube {
		return Cube {
			_position : postion,
			_angles: Vec3::new(0.0, 0.0, 0.0),

			_velocity: Vec3::new(0.0, 0.0, 0.0),
			_angle_velocity: Vec3::new(0.0, 0.0, 0.0),

			_width: width,
			_height: height
		}
	}
}