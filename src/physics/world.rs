use super::entity::Entity;

pub struct World {
	_entities: Vec<Box<dyn Entity>>,
	_gravity: f32
}

impl World {
    pub fn new() -> World {
        World { 
        	_entities: Vec::new(),
        	_gravity: 0.0
        }
    }

    pub fn set_gravity(&mut self, gravity_value : f32) {
    	self._gravity = gravity_value;
    }

    pub fn add_entity(&mut self, entity : Box<dyn Entity>) {
    	self._entities.push(entity);
    }

    pub fn simulate(&self, _buffer : &Vec<u32>) {

    }
}