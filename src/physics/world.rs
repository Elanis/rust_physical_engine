use super::entity::Entity;

use super::vec3::Vec3;

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

    		object.render(&mut _buffer, width, height);
    	}

        self.apply_collisions();
    }

    pub fn apply_collisions(&mut self) { 
        let mut new_velocities = Vec::new(); 
        new_velocities.resize(self._entities.len(), Vec3::new(0.0,0.0,0.0));

        for i in 0..(self._entities.len() - 1) {
            for j in 0..(self._entities.len() - 1) {
                if i == j { continue; }

                let first = &self._entities[i];
                let second = &self._entities[j];

                if first.get_position().distance(&second.get_position()) < (first.get_radius() + second.get_radius()) {
                    new_velocities[i] = -2.0 * first.get_velocity() * first.get_bouncing_value();
                    new_velocities[j] = -2.0 * second.get_velocity() * second.get_bouncing_value();
                }
            }
        }

        for i in 0..(self._entities.len() - 1) {
            self._entities[i].apply_velocity(new_velocities[i]);
        }
    }
}