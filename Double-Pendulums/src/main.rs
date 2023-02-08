use ::rand::prelude::*;
use double_pendulums::*;
use macroquad::prelude::*;

fn new_pendulums(n_pendulums: isize) -> Vec<Pendulum> {
    let mut rng = thread_rng();

    let mut pendulums: Vec<Pendulum> = Vec::new();
    for i in 0..n_pendulums {
        pendulums.push(Pendulum {
            bob1_mass: 10.0,
            bob2_mass: 10.0,
            arm1_length: 100.0,
            arm2_length: 100.0,
            arm1_angular_position: rng.gen_range(-6.28..6.28),
            arm2_angular_position: rng.gen_range(-6.28..6.28),
            arm1_angular_velocity: rng.gen_range(-3.14..3.14),
            arm2_angular_velocity: rng.gen_range(-3.14..3.14),
        });
    }

    return pendulums;
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

    let mut show_ui = false;

    let pendulums = new_pendulums(5);
    loop {
        clear_background(LIGHTGRAY);

        if show_ui == true {
            draw_energy();
        }

        for pendulum in &pendulums {
            draw_pendulum(&pendulum, x_centre, y_centre, radius, thickness);
        }
        draw_circle(x_centre, y_centre, 2.0, BLACK);

        next_frame().await
    }
}

fn draw_pendulum(pendulum: &Pendulum, x_centre: f32, y_centre: f32, radius: f32, thickness: f32) {
    // Unpacking coordinates and adding offset to render relative to center of screen
    let bob_coordinates = pendulum.bob_coordinates();
    let Coordinates(x1, y1) = bob_coordinates[0];
    let Coordinates(x2, y2) = bob_coordinates[1];
    let x1 = x1 + x_centre;
    let y1 = y1 + y_centre;
    let x2 = x2 + x_centre;
    let y2 = y2 + y_centre;

    // Draw circles at bobs and connect center to bob1, and bob1 to bob2
    draw_line(x_centre, y_centre, x1, y1, thickness, BLACK);
    draw_line(x1, y1, x2, y2, thickness, BLACK);
    draw_circle(x1, y1, radius, DARKGRAY);
    draw_circle(x2, y2, radius, DARKGRAY);
}
}
