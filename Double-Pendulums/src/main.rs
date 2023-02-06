use macroquad::prelude::*;

struct Coordinates(f32, f32);

struct Pendulum {
    // mass in kg
    bob1_mass: f64,
    bob2_mass: f64,
    // circular coordinates in f32 as macroquad draw functions use f32 inputs
    // length in m
    arm1_length: f32,
    arm2_length: f32,
    // angular position in rad, +ve is counter-clockwise
    arm1_angular_position: f32,
    arm2_angular_position: f32,
    // angular velocity in rad/s, +vs is counter-clockwise
    arm1_angular_velocity: f64,
    arm2_angular_velocity: f64,
}

impl Pendulum {
    fn bob_coordinates(&self) -> [Coordinates; 2] {
        let bob1_x = self.arm1_length * self.arm1_angular_position.sin();
        let bob1_y = self.arm1_length * self.arm1_angular_position.cos();
        let bob1_coordinates = Coordinates(bob1_x, bob1_y);

        let bob2_x = bob1_x + self.arm2_length * self.arm2_angular_position.sin();
        let bob2_y = bob1_y + self.arm2_length * self.arm2_angular_position.cos();
        let bob2_coordinates = Coordinates(bob2_x, bob2_y);

        return [bob1_coordinates, bob2_coordinates];
    }
}

#[macroquad::main("Double Pendulums")]
async fn main() {

    loop {

        next_frame().await
    }
}
