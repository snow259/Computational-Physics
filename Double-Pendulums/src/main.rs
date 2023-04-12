use std::f64::consts::PI;

// use ::rand::prelude::*;
use double_pendulums::*;
use macroquad::prelude::*;

fn new_pendulums(n_pendulums: isize, arm1_offset: f64, arm2_offset: f64) -> Vec<Pendulum> {
    // let mut rng = thread_rng();

    let mut pendulums: Vec<Pendulum> = Vec::new();
    for i in 0..n_pendulums {
        // pendulums.push(Pendulum {
        //     bob1_mass: 10.0,
        //     bob2_mass: 10.0,
        //     arm1_length: 100.0,
        //     arm2_length: 100.0,
        //     // arm1_angular_position: rng.gen_range(-6.28..6.28),
        //     // arm2_angular_position: rng.gen_range(-6.28..6.28),
        //     // arm1_angular_position: rng.gen_range(-2.0 * PI..2.0 * PI),
        //     // arm2_angular_position: rng.gen_range(-2.0 * PI..2.0 * PI),
        //     arm1_angular_position: 0.5 * PI,
        //     arm2_angular_position: 0.500000001 * PI,
        //     // arm1_angular_velocity: rng.gen_range(-PI..PI),
        //     // arm2_angular_velocity: rng.gen_range(-PI..PI),
        //     arm1_angular_velocity: 0.0,
        //     arm2_angular_velocity: 0.0,
        // });
        pendulums.push(Pendulum::new(
            10.0,
            10.0,
            100.0,
            100.0,
            0.5 * PI,
            0.500000001 * PI,
        ));
    }

    return pendulums;
}

fn draw_pendulum(pendulum: &Pendulum, x_centre: f32, y_centre: f32, radius: f32, thickness: f32) {
    // Adding offset to render relative to center of screen
    let bob_coordinates = pendulum.bob_coordinates();
    let x1 = x_centre + bob_coordinates.bob1_x as f32;
    let y1 = y_centre - bob_coordinates.bob1_y as f32;
    let x2 = x_centre + bob_coordinates.bob2_x as f32;
    let y2 = y_centre - bob_coordinates.bob2_y as f32;

    let bob_colour: Color;
    if pendulum.integrator == 0 {
        bob_colour = DARKGRAY;
    } else if pendulum.integrator == 1 {
        bob_colour = DARKBLUE;
    } else if pendulum.integrator == 2 {
        bob_colour = DARKGREEN;
    } else if pendulum.integrator == 3 {
        bob_colour = DARKPURPLE;
    } else {
        panic!("Invalid Pendulum.integrator");
    }
    // Draw circles at bobs and connect center to bob1, and bob1 to bob2
    draw_line(x_centre, y_centre, x1, y1, thickness, BLACK);
    draw_line(x1, y1, x2, y2, thickness, BLACK);
    draw_circle(x1, y1, radius, bob_colour);
    draw_circle(x2, y2, radius, bob_colour);
}

fn draw_energy(pendulums: &Vec<double_pendulums::Pendulum>) {
    let mut kinetic_energy = 0.0;
    let mut potential_energy = 0.0;
    let mut total_energy = 0.0;
    let mut max_potenital_energy = 0.0;
    let mut error_energy = 0.0;
    for pendulum in pendulums {
        kinetic_energy = kinetic_energy + pendulum.energy_kinetic();
        potential_energy = potential_energy + pendulum.energy_potential();
        total_energy = total_energy + pendulum.energy_total();
        max_potenital_energy = max_potenital_energy + pendulum.potential_energy_max;
        error_energy = error_energy + pendulum.energy_total() - pendulum.potential_energy_max;
    }

    draw_text(
        &format!("Kinetic Energy: {:.5} J", kinetic_energy),
        10.0,
        20.0,
        20.0,
        BLACK,
    );

    draw_text(
        &format!("Potential Energy: {:.5} J", potential_energy),
        10.0,
        40.0,
        20.0,
        BLACK,
    );

    draw_text(
        &format!("Total Energy: {:.5} J", total_energy),
        10.0,
        60.0,
        20.0,
        BLACK,
    );

    draw_text(
        &format!("Maximum Potenital Energy: {:.5} J", max_potenital_energy),
        10.0,
        80.0,
        20.0,
        BLACK,
    );

    draw_text(
        &format!("Error in Energy: {:.5} J", error_energy),
        10.0,
        100.0,
        20.0,
        BLACK,
    );
}

fn draw_time_step(t0: f64, x_centre: f32, y_centre: f32) {
    draw_text(
        &format!("t: {:.2} s", t0),
        x_centre * 2.0 - 100.0,
        20.0,
        20.0,
        BLACK,
    );
}

fn draw_fps(x_centre: f32, y_centre: f32) {
    draw_text(
        &format!("FPS: {:.2} Hz", get_fps()),
        x_centre * 2.0 - 100.0,
        40.0,
        20.0,
        BLACK,
    );
}

fn draw_paused(x_centre: f32, y_centre: f32) {
    let font_size = 20.0;
    draw_text(
        &format!("Paused"),
        // Font width appears to be about half the font size
        // "Paused" has 6 letters
        // Thus, font width / 2 for the mid point, * font size for the pixel offset
        x_centre - (3.0 / 2.0 * font_size),
        y_centre / 4.0,
        font_size,
        BLACK,
    );
}

fn toggle_state(state: &bool) -> bool {
    if state == &true {
        return false;
    } else {
        return true;
    }
}

fn window_conf() -> Conf {
    Conf {
        // .to_owned() converts &str to String
        window_title: "Double Pendulums".to_owned(),
        window_width: 1280,
        window_height: 720,
        // Fills remaining values with defaults
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Point thickness for pendulum bobs and base
    let radius = 5.0;
    // Thickness of pendulum arms
    let thickness = 1.0;

    let mut show_ui = true;
    let mut paused = true;

    // Starting time
    let mut t0 = 0.0;
    // t + dt per frame
    let frame_time_increment = 0.05;
    // Step size for integration over time
    let h = 0.0001;
    let method0 = "euler";
    let method1 = "semi_implicit_euler";
    let method2 = "rk4";
    let method3 = "leap_frog";

    let mut pendulums = new_pendulums(1, 0.0, 0.0);
    loop {
        clear_background(LIGHTGRAY);
        let x_centre = screen_width() / 2.0;
        let y_centre = screen_height() / 2.0;

        if show_ui == true {
            draw_energy(&pendulums);
            draw_time_step(t0, x_centre, y_centre);
            draw_fps(x_centre, y_centre);

            if paused == true {
                draw_paused(x_centre, y_centre);
            }
        }

        if is_key_pressed(KeyCode::F2) {
            show_ui = toggle_state(&show_ui);
        }
        if is_key_pressed(KeyCode::Space) {
            paused = toggle_state(&paused);
        }

        if paused == false {
            for pendulum in &mut pendulums {
                pendulum.update(t0, t0 + frame_time_increment, h, method2);
            }
            // pendulums[0].update(t0, t0 + frame_time_increment, h, method2);

            // pendulums[0].update(t0, t0 + frame_time_increment, h, method0);
            // pendulums[1].update(t0, t0 + frame_time_increment, h, method1);
            // pendulums[2].update(t0, t0 + frame_time_increment, h, method2);
            // pendulums[3].update(t0, t0 + frame_time_increment, h, method3);
            t0 = t0 + frame_time_increment;
        }

        for pendulum in &pendulums {
            draw_pendulum(&pendulum, x_centre, y_centre, radius, thickness);
        }
        draw_circle(x_centre, y_centre, 2.0, BLACK);

        next_frame().await
    }
}
