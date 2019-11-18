use super::vec3::Vec3;

pub trait Entity {
	fn get_velocity(&self) -> Vec3;
	fn get_angle_velocity(&self) -> Vec3;

	fn render(&self, _buffer : &mut Vec<u32>, width : usize, height : usize);

	fn recalc_speed(&mut self, tick : f32, gravity : f32);
	fn recalc_pos(&mut self, tick : f32);

	fn apply_collisions(&mut self, width : usize, height : usize);

	
	fn get_up(&self) -> f32;
	fn get_left(&self) -> f32;
	fn get_bottom(&self) -> f32;
	fn get_right(&self) -> f32;
}