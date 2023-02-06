use ::rand::prelude::*;
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
    let mut rng = thread_rng();
    let radius = 5.0;
    let x_centre = screen_width() / 2.0;
    let y_centre = screen_height() / 2.0;

    let pendulum = Pendulum {
        bob1_mass: 10.0,
        bob2_mass: 10.0,
        arm1_length: 50.0,
        arm2_length: 50.0,
        arm1_angular_position: rng.gen_range(-6.28..6.28),
        arm2_angular_position: rng.gen_range(-6.28..6.28),
        arm1_angular_velocity: 1.0,
        arm2_angular_velocity: -0.5,
    };

    loop {
        draw_circle(x_centre, y_centre, radius, RED);
        draw_pendulum(&pendulum, x_centre, y_centre);

        next_frame().await
    }
}

fn draw_pendulum(pendulum: &Pendulum, x_centre: f32, y_centre: f32) {
    let Coordinates(x1, y1) = pendulum.bob_coordinates()[0];
    let Coordinates(x2, y2) = pendulum.bob_coordinates()[1];
    let radius = 5.0;
    let thickness = 0.5;
    let x1 = x1 + x_centre;
    let y1 = y1 + y_centre;
    let x2 = x2 + x_centre;
    let y2 = y2 + y_centre;

    draw_circle(x1, y1, radius, BLUE);
    draw_circle(x2, y2, radius, BLUE);
    draw_line(x_centre, y_centre, x1, y1, thickness, BLUE);
    draw_line(x1, y1, x2, y2, thickness, BLUE);
}
