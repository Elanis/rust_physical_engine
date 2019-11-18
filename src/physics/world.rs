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

    pub fn simulate(&mut self, mut _buffer : &mut Vec<u32>, width : usize, height : usize) {
        let tick : f32 = 1.0/30.0;

    	for (_i, object) in self._entities.iter_mut().enumerate() {
            object.recalc_speed(tick, self._gravity);
            object.recalc_pos(tick);

            object.apply_collisions(width, height);

    		object.render(&mut _buffer, width, height);
    	}
    }
}