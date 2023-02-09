pub const G: f32 = 9.81;

pub struct Coordinates(pub f32, pub f32);

#[derive(Debug)]
pub struct BobCoordinates {
    // Without BobCoordinates, there was repeated unpacking of coordinates across the code
    pub bob1_x: f32,
    pub bob1_y: f32,
    pub bob2_x: f32,
    pub bob2_y: f32,
}

impl PartialEq for BobCoordinates {
    fn eq(&self, other: &Self) -> bool {
        self.bob1_x == other.bob1_x
            && self.bob1_y == other.bob1_y
            && self.bob2_x == other.bob2_x
            && self.bob2_y == other.bob2_y
    }
}

pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn dot(&self, v: Vector2) -> f32 {
        return self.x * v.x + self.y * v.y;
    }

    pub fn cross(&self, v: Vector2) -> f32 {
        // returns f32 despite being cross product as a two dimensional cross product has only one non zero term in three dimensions
        return self.x * v.y - self.y * v.x;
    }
}

#[derive(Default)]
pub struct Pendulum {
    // Any variables using f32 are because macroquad draw functions use f32 inputs
    // mass in kg
    pub bob1_mass: f32,
    pub bob2_mass: f32,
    // length in m
    pub arm1_length: f32,
    pub arm2_length: f32,
    // angular position in rad, +ve is counter-clockwise
    pub arm1_angular_position: f32,
    pub arm2_angular_position: f32,
    // angular velocity in rad/s, +vs is counter-clockwise
    pub arm1_angular_velocity: f32,
    pub arm2_angular_velocity: f32,
}

impl Pendulum {
    pub fn bob_coordinates(&self) -> BobCoordinates {
        let bob1_x = self.arm1_length * self.arm1_angular_position.cos();
        let bob1_y = self.arm1_length * self.arm1_angular_position.sin();

        let bob2_x = bob1_x + self.arm2_length * self.arm2_angular_position.cos();
        let bob2_y = bob1_y + self.arm2_length * self.arm2_angular_position.sin();

        return BobCoordinates {
            bob1_x,
            bob1_y,
            bob2_x,
            bob2_y,
        };
    }

    pub fn energy_kinetic(&self) -> f32 {
        let moment_of_intertia1 = self.bob1_mass * self.arm1_length;
        let moment_of_intertia2 = self.bob2_mass * self.arm2_length;
        let mut kinetic_energy: f32 = 0.0;

        kinetic_energy = kinetic_energy + moment_of_intertia1 * self.arm1_angular_velocity.powi(2);
        kinetic_energy = kinetic_energy + moment_of_intertia2 * self.arm2_angular_velocity.powi(2);
        kinetic_energy = kinetic_energy
            + self.bob1_mass * (self.arm1_length * self.arm1_angular_velocity).powi(2);
        kinetic_energy = kinetic_energy
            + self.bob2_mass * (self.arm2_length * self.arm2_angular_velocity).powi(2);

        return kinetic_energy * 0.5;
    }

    pub fn energy_potential(&self) -> f32 {
        let height_pendulum_base = self.arm1_length + self.arm2_length;
        let bob_coordinates = self.bob_coordinates();

        let potential_energy = self.bob1_mass * (height_pendulum_base + bob_coordinates.bob1_y)
            + self.bob2_mass * (height_pendulum_base + bob_coordinates.bob2_y);

        return potential_energy * G;
    }

    pub fn energy_total(&self) -> f32 {
        let total_energy = self.energy_potential() + self.energy_kinetic();

        return total_energy;
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use crate::Pendulum;

    const FLOAT_TOLERANCE: f32 = 0.000001;

    #[test]
    fn test_bob_coordinates() {
        let pendulum1 = Pendulum {
            arm1_length: 10.0,
            arm2_length: 10.0,
            arm1_angular_position: 0.0,
            arm2_angular_position: 0.0,
            ..Default::default()
        };

        let pendulum2 = Pendulum {
            arm1_length: 10.0,
            arm2_length: 10.0,
            arm1_angular_position: 1.5 * PI,
            arm2_angular_position: 1.5 * PI,
            ..Default::default()
        };

        assert_eq!(pendulum1.bob_coordinates().bob1_x, 10.0);
        assert_eq!(pendulum1.bob_coordinates().bob1_y, 0.0);
        assert_eq!(pendulum1.bob_coordinates().bob2_x, 20.0);
        assert_eq!(pendulum1.bob_coordinates().bob2_y, 0.0);

        assert!((pendulum2.bob_coordinates().bob1_x - 0.0).abs() < FLOAT_TOLERANCE);
        assert!((pendulum2.bob_coordinates().bob1_y - (-10.0)).abs() < FLOAT_TOLERANCE);
        assert!((pendulum2.bob_coordinates().bob2_x - 0.0).abs() < FLOAT_TOLERANCE);
        assert!((pendulum2.bob_coordinates().bob2_y - (-20.0)).abs() < FLOAT_TOLERANCE);
    }
}
