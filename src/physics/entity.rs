use super::vec3::Vec3;

pub trait Entity {
	fn get_velocity(&self) -> Vec3; 
	fn get_angle_velocity(&self) -> Vec3; 
}