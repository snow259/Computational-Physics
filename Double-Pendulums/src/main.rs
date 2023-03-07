use std::f64::consts::PI;

// use ::rand::prelude::*;
use double_pendulums::*;
use macroquad::prelude::*;

fn new_pendulums(n_pendulums: isize) -> Vec<Pendulum> {
    // let mut rng = thread_rng();

    let mut pendulums: Vec<Pendulum> = Vec::new();
    for _i in 0..n_pendulums {
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

    // Draw circles at bobs and connect center to bob1, and bob1 to bob2
    draw_line(x_centre, y_centre, x1, y1, thickness, BLACK);
    draw_line(x1, y1, x2, y2, thickness, BLACK);
    draw_circle(x1, y1, radius, DARKGRAY);
    draw_circle(x2, y2, radius, DARKGRAY);
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

fn toggle_ui(show_ui: &bool) -> bool {
    if show_ui == &true {
        return false;
    } else {
        return true;
    }
}

fn toggle_pause(pause: &bool) -> bool {
    if pause == &true {
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
    let x_centre = screen_width() / 2.0;
    let y_centre = screen_height() / 2.0;
    let radius = 5.0;
    let thickness = 1.0;

    let mut show_ui = true;
    let mut pause = true;

    let mut t0 = 0.0;
    let frame_time_increment = 0.05;
    let h = 0.01;
    let method0 = "euler";
    let method1 = "semi_implicit_euler";
    let method2 = "rk4";
    let method3 = "leap_frog";

    let mut pendulums = new_pendulums(1);
    loop {
        clear_background(LIGHTGRAY);

        if show_ui == true {
            draw_energy(&pendulums);
        }

        if is_key_pressed(KeyCode::F2) {
            show_ui = toggle_ui(&show_ui);
        }
        if is_key_pressed(KeyCode::Space) {
            pause = toggle_pause(&pause);
        }

        if pause == false {
            pendulums[0].update(t0, t0 + frame_time_increment, h, method2);

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
