use super::vec3::Vec3;

use super::entity::Entity;

pub trait Positionnable : Entity {
	fn get_position(&self) -> Vec3;
	fn get_angles(&self) -> Vec3;
}