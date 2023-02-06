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

#[macroquad::main("Double Pendulums")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    let mut x_increment = 1.0;
    let mut y_increment = 1.0;
    let radius = 25.0;
    loop {
        x = x + x_increment;
        y = y + y_increment;

        if x + radius >= screen_width() {
            x_increment = x_increment * -1.0;
        }

        if x - radius <= 0.0 {
            x_increment = x_increment * -1.0;
        }

        if y + radius >= screen_height() {
            y_increment = y_increment * -1.0;
        }

        if y - radius <= 0.0 {
            y_increment = y_increment * -1.0;
        }

        draw_circle(x, y, radius, BLUE);
        next_frame().await
    }
}
