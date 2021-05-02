#![allow(warnings)]
use macroquad::prelude::*;

use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Ui,
};

#[derive(Debug, Clone)]
pub struct Boid {
    pub id: u32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub color: Color,
}

impl Boid {
    pub fn update(&mut self) {
        self.position += self.velocity;

        if self.position.x > 640. {
            self.position.x = 0.;
        }
        if self.position.x < 0. {
            self.position.x = 640.;
        }

        if self.position.y > 480. {
            self.position.y = 0.;
        }
        if self.position.y < 0. {
            self.position.y = 480.;
        }
    }
    pub fn render(&self) {
       // draw_poly(self.position.x, self.position.y, 3, 5.0, 0., self.color);
         draw_circle(self.position.x, self.position.y, 3., self.color);
    }
    pub fn length(a: Vec2) -> f32 {
        return f32::sqrt((a.x * a.x) as f32 + (a.y * a.y) as f32);
    }
}

#[derive(Debug, Clone)]
pub struct BoidSystem {
    pub boids: Vec<Boid>,
}

impl BoidSystem {
    pub fn new(total_boids: u32) -> BoidSystem {
        let mut boids_list = Vec::new();

        for i in 0..total_boids {
            boids_list.push(Boid {
                id: i,
                position: vec2(rand::gen_range(0.0, 640.), rand::gen_range(0.0, 480.)),
                velocity: vec2(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)),
                color: WHITE,
            });
        }

        BoidSystem { boids: boids_list }
    }

    pub fn get_neighbours(&self, current_boid: &Boid) -> Vec<Boid> {
        let mut neighbours: Vec<Boid> = Vec::new();

        for other_boid in self.boids.iter().cloned() {
            if other_boid.id != current_boid.id {
                neighbours.push(other_boid);
            }
        }
        neighbours
    }
    fn length(vector: Vec2) -> f32 {
        f32::sqrt((vector.x * vector.x) + (vector.y * vector.y))
    }
    fn distance(boid1: Vec2, boid2: Vec2) -> f32 {
        f32::sqrt(
            (boid1.x - boid2.x) * (boid1.x - boid2.x) + (boid1.y - boid2.y) * (boid1.y - boid2.y),
        )
    }
    pub fn update(
        &mut self,
        cohesion_val: f32,
        velocity_val: f32,
        fov: f32,
        separation_val: f32,
        allign_val: f32,
    ) {
        //visualising the first boid
        let r: Vec2 = self.boids[0].position;
        draw_circle(
            r.x,
            r.y,
            fov,
            Color {
                r: 255.,
                g: 0.,
                b: 0.,
                a: 0.23,
            },
        );

        //loop through and apply the boid rules
        for i in 0..self.boids.len() {
            let mut boid = self.boids[i].clone();
            let neighbours = self.clone().get_neighbours(&boid);

            //cohesion
            let mut c_vec: Vec2 = vec2(0., 0.);
            for j in 0..neighbours.len() {
                if BoidSystem::distance(boid.position, neighbours[j].position) < fov {
                    c_vec += neighbours[j].position;
                }
            }
            c_vec /= neighbours.len() as f32;
            if cohesion_val != 0.0 {
                boid.velocity += (c_vec - boid.position) / (7000.0 / cohesion_val);
            }
            //separation
            let mut s_vec: Vec2 = vec2(0., 0.);
            for j in 0..neighbours.len() {
                if BoidSystem::distance(boid.position, neighbours[j].position) < fov {
                    if BoidSystem::length(neighbours[j].position - boid.position) < separation_val {
                        s_vec -= neighbours[j].position - boid.position;
                    }
                }
            }
            boid.velocity += s_vec;
            //allignment
            let mut a_vec: Vec2 = vec2(0., 0.); 
            for j in 0..neighbours.len() {
                if BoidSystem::distance(boid.position, neighbours[j].position) < fov {
                    c_vec += neighbours[j].velocity;
                }
            }
            c_vec /= neighbours.len() as f32;
            if allign_val != 0.0 {
                boid.velocity += (c_vec - boid.velocity) / allign_val;
            }
            //limit velocity
            if BoidSystem::length(boid.velocity) > velocity_val {
                boid.velocity = (boid.velocity / BoidSystem::length(boid.velocity)) * velocity_val;
            }

            boid.update();
            boid.render();
            self.boids[i] = boid;
        }
    }
}

