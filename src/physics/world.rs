use std::time::UNIX_EPOCH;
use std::time::{SystemTime};

use std::{thread, time};

use super::entity::Entity;

use super::vec3::Vec3;

pub struct World {
	_entities: Vec<Box<dyn Entity>>,
	_gravity: f32,
    _last_tick: u128,
}

impl World {
    pub fn new() -> World {
        World { 
        	_entities: Vec::new(),
        	_gravity: 0.0,
            _last_tick: World::get_curr_tick()
        }
    }

    fn get_curr_tick() -> u128 {
        let start = SystemTime::now();

        match start.duration_since(UNIX_EPOCH) {
            Ok(elapsed) => {
                return elapsed.as_millis();
            }
            Err(e) => {
                // an error occured!
                println!("Error: {:?}", e);
                return 0;
            }
        }
    }

    pub fn set_gravity(&mut self, gravity_value : f32) {
    	self._gravity = gravity_value;
    }

    pub fn add_entity(&mut self, entity : Box<dyn Entity>) {
    	self._entities.push(entity);
    }

    pub fn simulate(&mut self, mut _buffer : &mut Vec<u32>, width : usize, height : usize) -> f32 {
        let tick : f32 = (World::get_curr_tick() - self._last_tick) as f32 / 1_000.0;
        self._last_tick = World::get_curr_tick();

        println!("Tick length: {0} s - Estimated FPS: {1}", tick, (1000.0 / tick).round());

    	for (_i, object) in self._entities.iter_mut().enumerate() {
            object.recalc_speed(tick, self._gravity);
    	}

        self.apply_collisions();

        for (_i, object) in self._entities.iter_mut().enumerate() {
            object.recalc_pos(tick);

            object.render(&mut _buffer, width, height);
        }

        (1000.0 / tick).round()
    }

    pub fn collides_as_box(a : &Box<dyn Entity>, b : &Box<dyn Entity>) -> bool {
        (
            a.get_up() < b.get_bottom() && a.get_up() > b.get_up() ||
            b.get_up() < a.get_bottom() && b.get_up() > a.get_up()
        ) && (
            a.get_left() < b.get_right() && a.get_left() > b.get_left() ||
            b.get_left() < a.get_right() && b.get_left() > a.get_left()
        )
    }

    pub fn collides(a : &Box<dyn Entity>, b : &Box<dyn Entity>) -> bool {
        (
            a.get_position().distance(&b.get_position()) < (a.get_radius() + b.get_radius()) ||
            World::collides_as_box(&a, &b) ||
            World::collides_as_box(&b, &a)
        )
    }

    pub fn apply_collisions(&mut self) { 
        let mut new_velocities = Vec::new(); 
        new_velocities.resize(self._entities.len(), Vec3::new(0.0,0.0,0.0));

        for i in 0..self._entities.len() {
            for j in 0..self._entities.len() {
                if i <= j { continue; }

                let first = &self._entities[i];
                let second = &self._entities[j];

                if World::collides(&first, &second) {

                    let first_velocity = -2.0 * first.get_velocity() * first.get_bouncing_value();
                    let second_velocity = -2.0 * second.get_velocity() * second.get_bouncing_value();

                    self._entities[i].apply_velocity(first_velocity);
                    self._entities[j].apply_velocity(second_velocity);
                }
            }
        }
    }
}