use ::rand::prelude::*;
use macroquad::prelude::*;

pub const G: f32 = 6.67430 / 100_000_000_000.0;
pub const H: f32 = 0.01;
pub struct VectorArray {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
}

fn generate_mass_vectors(n: u32) -> Vec<f32> {
    let mut mass_vectors: Vec<f32> = Vec::new();
    let mut rng = thread_rng();

    for _i in 0..n {
        mass_vectors.push(rng.gen());
    }

    for i in 0..mass_vectors.len() {
        mass_vectors[i] = G * mass_vectors[i] * 10_f32.powi(15);
    }

    return mass_vectors;
}

fn generate_velocity_vectors(n: u32) -> VectorArray {
    let mut velocity_vectors = VectorArray {
        x: Vec::new(),
        y: Vec::new(),
    };
    let mut rng = thread_rng();

    for _i in 0..n {
        let vel_x: f32 = (rng.gen::<f32>() - 0.5) * 5.0;
        let vel_y: f32 = (rng.gen::<f32>() - 0.5) * 5.0;
        velocity_vectors.x.push(vel_x);
        velocity_vectors.y.push(vel_y);
    }

    return velocity_vectors;
}

fn generate_position_vectors(n: u32) -> VectorArray {
    let mut position_vectors = VectorArray {
        x: Vec::new(),
        y: Vec::new(),
    };
    let mut rng = thread_rng();

    for _i in 0..n {
        position_vectors
            .x
            .push((rng.gen::<f32>() - 0.5) * screen_width() * 0.5 + screen_width() * 0.5);
        position_vectors
            .y
            .push((rng.gen::<f32>() - 0.5) * screen_height() * 0.5 + screen_height() * 0.5);
    }

    return position_vectors;
}

fn generate_acc_vectors(position_vectors: &VectorArray, mass_vectors: &Vec<f32>) -> VectorArray {
    let mut acc_vectors = VectorArray {
        x: Vec::new(),
        y: Vec::new(),
    };
    let n = mass_vectors.len();

    for i in 0..n {
        let mut acc_x = 0.0;
        let mut acc_y = 0.0;

        for j in 0..n {
            if j != i {
                let acc = compute_m_by_r_sq(
                    mass_vectors[j],
                    position_vectors.x[i],
                    position_vectors.y[i],
                    position_vectors.x[j],
                    position_vectors.y[j],
                );
                acc_x = acc_x + acc.0;
                acc_y = acc_y + acc.1;
            }
        }
        acc_vectors.x.push(acc_x);
        acc_vectors.y.push(acc_y);
    }

    return acc_vectors;
}

fn compute_m_by_r_sq(m: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> (f32, f32) {
    let epsilon: f32 = 5.0;
    let rx = x2 - x1;
    let ry = y2 - y1;
    let rmag_sq = rx.powi(2) + ry.powi(2);
    let a = m / (rmag_sq + epsilon.powi(2)).powf(3.0 / 2.0);

    return (a * rx, a * ry);
}

fn update_euler<'a>(
    position_vectors: &'a mut VectorArray,
    velocity_vectors: &'a mut VectorArray,
    acc_vectors: &'a VectorArray,
) {
    for i in 0..position_vectors.x.len() {
        // position_vectors.x[i] = 100.0;
        velocity_vectors.x[i] = velocity_vectors.x[i] + H * acc_vectors.x[i];
        velocity_vectors.y[i] = velocity_vectors.y[i] + H * acc_vectors.y[i];

        position_vectors.x[i] = position_vectors.x[i] + H * velocity_vectors.x[i];
        position_vectors.y[i] = position_vectors.y[i] + H * velocity_vectors.y[i];
    }

    // return (position_vectors, velocity_vectors);
}

fn draw_particles(position_vectors: &VectorArray) {
    let circle_size = 1.0;
    for i in 0..position_vectors.x.len() {
        draw_circle(
            position_vectors.x[i],
            position_vectors.y[i],
            circle_size,
            WHITE,
        );
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Simple NBody".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let n = 500;
    let mass_vectors = generate_mass_vectors(n);
    let mut velocity_vectors = generate_velocity_vectors(n);
    let mut position_vectors = generate_position_vectors(n);

    // print_vector_array(&position_vectors, &"Position Vectors".to_owned());
    // print_vector_array(&velocity_vectors, &"Velocity Vectors".to_owned());

    loop {
        let acc_vectors = generate_acc_vectors(&position_vectors, &mass_vectors);
        // let (&position_vectors, &velocity_vectors) =  -> (&'a VectorArray, &'a VectorArray)

        // if is_key_pressed(KeyCode::Space) {
        update_euler(&mut position_vectors, &mut velocity_vectors, &acc_vectors);
        // print_vector_array(&position_vectors, &"Position Vectors".to_owned());
        // print_vector_array(&velocity_vectors, &"Velocity Vectors".to_owned());
        // }

        clear_background(BLACK);
        draw_particles(&position_vectors);
        next_frame().await
    }
}

fn print_vector_array(vector_array: &VectorArray, name: &String) {
    print!("{name}\n");
    for i in 0..vector_array.x.len() {
        let x = vector_array.x[i];
        let y = vector_array.y[i];
        print!("{i}: x = {x}, y = {y}\n");
    }
}

#[cfg(test)]
mod tests {
    use crate::{compute_m_by_r_sq, G};

    const FLOAT_TOLERANCE: f32 = 0.000_1;

    #[test]
    fn test_compute_m_by_r_sq() {
        let earth_mass = 3.986004418 * 10_f32.powi(14);
        let earth_radius = 6371.0 * 10_f32.powi(3);
        let earth_surface_acc = 9.82025;

        let (acc_x, acc_y) = compute_m_by_r_sq(earth_mass, earth_radius, 0.0, 0.0, 0.0);
        print!("{acc_x}, {acc_y}");

        assert!((acc_x + earth_surface_acc).abs() < FLOAT_TOLERANCE);
        assert!((acc_y - 0.0).abs() < FLOAT_TOLERANCE);

        let (acc_x, acc_y) = compute_m_by_r_sq(earth_mass, 0.0, earth_radius, 0.0, 0.0);
        assert!((acc_x - 0.0).abs() < FLOAT_TOLERANCE);
        assert!((acc_y + earth_surface_acc).abs() < FLOAT_TOLERANCE);
    }
}
