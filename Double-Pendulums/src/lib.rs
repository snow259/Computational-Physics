pub const G: f64 = 9.81;

pub struct Coordinates(pub f32, pub f32);

#[derive(Debug)]
pub struct BobCoordinates {
    // Without BobCoordinates, there was repeated unpacking of coordinates across the code
    pub bob1_x: f64,
    pub bob1_y: f64,
    pub bob2_x: f64,
    pub bob2_y: f64,
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
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn dot(&self, v: Vector2) -> f64 {
        return self.x * v.x + self.y * v.y;
    }

    pub fn cross(&self, v: Vector2) -> f64 {
        // returns f64 despite being cross product as a two dimensional cross product has only one non zero term in three dimensions
        return self.x * v.y - self.y * v.x;
    }
}

#[derive(Default)]
pub struct Pendulum {
    // mass in kg
    pub bob1_mass: f64,
    pub bob2_mass: f64,
    // length in m
    pub arm1_length: f64,
    pub arm2_length: f64,
    // angular position in rad, +ve is counter-clockwise, vertically down is 0
    pub arm1_angular_position: f64,
    pub arm2_angular_position: f64,
    // angular velocity in rad/s, +vs is counter-clockwise
    pub arm1_angular_velocity: f64,
    pub arm2_angular_velocity: f64,
}

impl Pendulum {
    pub fn bob_coordinates(&self) -> BobCoordinates {
        // Coordinates are returned keeping origin at the base of the pendulum
        let bob1_x = self.arm1_length * self.arm1_angular_position.sin();
        let bob1_y = -self.arm1_length * self.arm1_angular_position.cos();

        let bob2_x = bob1_x + self.arm2_length * self.arm2_angular_position.sin();
        let bob2_y = bob1_y - self.arm2_length * self.arm2_angular_position.cos();

        return BobCoordinates {
            bob1_x,
            bob1_y,
            bob2_x,
            bob2_y,
        };
    }

    pub fn energy_kinetic(&self) -> f64 {
        // Kinetic energy formula comes from https://en.wikipedia.org/wiki/Double_pendulum#Analysis_and_interpretation
        let moment_of_intertia1 = self.bob1_mass * self.arm1_length.powi(2);
        let moment_of_intertia2 = self.bob2_mass * self.arm2_length.powi(2);
        let mut kinetic_energy: f64 = 0.0;

        kinetic_energy = kinetic_energy + moment_of_intertia1 * self.arm1_angular_velocity.powi(2);
        kinetic_energy = kinetic_energy + moment_of_intertia2 * self.arm2_angular_velocity.powi(2);
        kinetic_energy = kinetic_energy
            + self.bob1_mass * (self.arm1_length * self.arm1_angular_velocity).powi(2);
        kinetic_energy = kinetic_energy
            + self.bob2_mass * (self.arm2_length * self.arm2_angular_velocity).powi(2);

        return kinetic_energy * 0.5;
    }

    pub fn energy_potential(&self) -> f64 {
        // Potential energy is calculated after shifting origin to lower most extent a pendulum can go
        // This is treated as the ground, and since all pendulums in the simulation have the same arm lengths, is the same for all pendulums
        let height_pendulum_base = self.arm1_length + self.arm2_length;
        let bob_coordinates = self.bob_coordinates();

        let potential_energy = self.bob1_mass * (height_pendulum_base + bob_coordinates.bob1_y)
            + self.bob2_mass * (height_pendulum_base + bob_coordinates.bob2_y);

        return potential_energy * G;
    }

    pub fn energy_total(&self) -> f64 {
        let total_energy = self.energy_potential() + self.energy_kinetic();

        return total_energy;
    }

    pub fn acc_arm1(&self) -> f64 {
        let term1 =
            -G * (2.0 * self.bob1_mass + self.bob2_mass) * (self.arm1_angular_position).sin();

        let term2 = -(self.bob2_mass * G)
            * (self.arm1_angular_position - 2.0 * self.arm2_angular_position).sin();

        let term3 = -2.0
            * (self.arm1_angular_position - self.arm2_angular_position).sin()
            * self.bob2_mass
            * (self.arm2_angular_velocity.powi(2) * self.arm2_length
                + self.arm1_angular_velocity.powi(2)
                    * self.arm1_length
                    * (self.arm1_angular_position - self.arm2_angular_position).cos());

        let denominator = self.arm1_length
            * (2.0 * self.bob1_mass + self.bob2_mass
                - self.bob2_mass
                    * (2.0 * self.arm1_angular_position - 2.0 * self.arm2_angular_position).cos());

        return (term1 + term2 + term3) / denominator;
    }

    pub fn acc_arm2(&self) -> f64 {
        let term1 = 2.0 * (self.arm1_angular_position - self.arm2_angular_position).sin();

        let term2 = self.arm1_angular_velocity.powi(2)
            * self.arm1_length
            * (self.bob1_mass + self.bob2_mass);

        let term3 = G * (self.bob1_mass + self.bob2_mass) * (self.arm1_angular_position).cos();

        let term4 = self.arm2_angular_velocity.powi(2) * self.arm2_length
            + self.bob2_mass * (self.arm1_angular_position - self.arm2_angular_position).cos();

        let denominator = self.arm2_length
            * (2.0 * self.bob1_mass + self.bob2_mass
                - self.bob2_mass
                    * (2.0 * self.arm1_angular_position - 2.0 * self.arm2_angular_position).cos());

        return (term1 * (term2 + term3 + term4)) / denominator;
    }

    pub fn update(&mut self, h: f64) {
        let new_arm1_angular_velocity = self.arm1_angular_velocity + h * self.acc_arm1();
        let new_arm2_angular_velocity = self.arm2_angular_velocity + h * self.acc_arm2();
        let new_arm1_angular_position = self.arm1_angular_position + h * self.arm1_angular_velocity;
        let new_arm2_angular_position = self.arm2_angular_position + h * self.arm2_angular_velocity;

        self.arm1_angular_velocity = new_arm1_angular_velocity;
        self.arm2_angular_velocity = new_arm2_angular_velocity;
        self.arm1_angular_position = new_arm1_angular_position;
        self.arm2_angular_position = new_arm2_angular_position;
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::Pendulum;

    const FLOAT_TOLERANCE: f64 = 0.000_000_000_000_1;

    #[test]
    fn test_bob_coordinates() {
        let pendulum1 = Pendulum {
            arm1_length: 10.0,
            arm2_length: 10.0,
            arm1_angular_position: 0.5 * PI,
            arm2_angular_position: 0.5 * PI,
            ..Default::default()
        };

        let pendulum2 = Pendulum {
            arm1_length: 10.0,
            arm2_length: 10.0,
            arm1_angular_position: 0.0 * PI,
            arm2_angular_position: 0.0 * PI,
            ..Default::default()
        };

        assert!(pendulum1.bob_coordinates().bob1_x - 10.0 < FLOAT_TOLERANCE);
        assert!(pendulum1.bob_coordinates().bob1_y - 0.0 < FLOAT_TOLERANCE);
        assert!(pendulum1.bob_coordinates().bob2_x - 20.0 < FLOAT_TOLERANCE);
        assert!(pendulum1.bob_coordinates().bob2_y - 0.0 < FLOAT_TOLERANCE);

        assert!((pendulum2.bob_coordinates().bob1_x - 0.0).abs() < FLOAT_TOLERANCE);
        assert!((pendulum2.bob_coordinates().bob1_y - (-10.0)).abs() < FLOAT_TOLERANCE);
        assert!((pendulum2.bob_coordinates().bob2_x - 0.0).abs() < FLOAT_TOLERANCE);
        assert!((pendulum2.bob_coordinates().bob2_y - (-20.0)).abs() < FLOAT_TOLERANCE);
    }

    #[test]
    fn test_energy_kinetic() {
        let pendulum1 = Pendulum {
            arm1_length: 10.0,
            arm2_length: 10.0,
            bob1_mass: 10.0,
            bob2_mass: 10.0,
            arm1_angular_velocity: 1.0,
            arm2_angular_velocity: 1.0,
            ..Default::default()
        };

        let pendulum2 = Pendulum {
            arm1_length: 10.0,
            arm2_length: 10.0,
            bob1_mass: 10.0,
            bob2_mass: 10.0,
            arm1_angular_velocity: -1.0,
            arm2_angular_velocity: 0.0,
            ..Default::default()
        };

        assert!((pendulum1.energy_kinetic() - 2000.0).abs() < FLOAT_TOLERANCE);
        assert!((pendulum2.energy_kinetic() - 1000.0).abs() < FLOAT_TOLERANCE);
    }

    #[test]
    fn test_energy_potential() {
        let pendulum1 = Pendulum {
            arm1_length: 10.0,
            arm2_length: 10.0,
            bob1_mass: 10.0,
            bob2_mass: 10.0,
            arm1_angular_position: 0.0 * PI,
            arm2_angular_position: 0.0 * PI,
            ..Default::default()
        };

        let pendulum2 = Pendulum {
            arm1_length: 10.0,
            arm2_length: 10.0,
            bob1_mass: 10.0,
            bob2_mass: 10.0,
            arm1_angular_position: 0.5 * PI,
            arm2_angular_position: 0.5 * PI,
            ..Default::default()
        };

        assert!((pendulum1.energy_potential() - 981.0).abs() < FLOAT_TOLERANCE);
        assert!((pendulum2.energy_potential() - 3924.0).abs() < FLOAT_TOLERANCE);
    }

    #[test]
    fn test_energy_total() {
        let pendulum1 = Pendulum {
            arm1_length: 10.0,
            arm2_length: 10.0,
            bob1_mass: 10.0,
            bob2_mass: 10.0,
            arm1_angular_position: 0.0,
            arm2_angular_position: 0.0,
            arm1_angular_velocity: 1.0,
            arm2_angular_velocity: 1.0,
        };

        let pendulum2 = Pendulum {
            arm1_length: 10.0,
            arm2_length: 10.0,
            bob1_mass: 10.0,
            bob2_mass: 10.0,
            arm1_angular_position: 0.0,
            arm2_angular_position: 0.0,
            arm1_angular_velocity: -1.0,
            arm2_angular_velocity: 0.0,
        };

        assert!(
            (pendulum1.energy_total() - pendulum1.energy_kinetic() - pendulum1.energy_potential())
                .abs()
                < FLOAT_TOLERANCE
        );
        assert!(
            (pendulum2.energy_total() - pendulum2.energy_kinetic() - pendulum2.energy_potential())
                .abs()
                < FLOAT_TOLERANCE
        );
    }

    #[test]
    fn test_acc_arm1() {
        let pendulum1 = Pendulum {
            arm1_length: 10.0,
            arm2_length: 10.0,
            bob1_mass: 10.0,
            bob2_mass: 10.0,
            arm1_angular_position: 0.0 * PI,
            arm2_angular_position: 0.0 * PI,
            arm1_angular_velocity: 0.0,
            arm2_angular_velocity: 0.0,
        };

        let pendulum2 = Pendulum {
            arm1_length: 10.0,
            arm2_length: 10.0,
            bob1_mass: 10.0,
            bob2_mass: 10.0,
            arm1_angular_position: 0.5 * PI,
            arm2_angular_position: 0.5 * PI,
            arm1_angular_velocity: 0.0,
            arm2_angular_velocity: 0.0,
        };

        assert!(pendulum1.acc_arm1() - 0.0 < FLOAT_TOLERANCE);
        assert!(pendulum2.acc_arm1() - 0.0 < FLOAT_TOLERANCE);
    }

    #[test]
    fn test_acc_arm2() {
        let pendulum1 = Pendulum {
            arm1_length: 10.0,
            arm2_length: 10.0,
            bob1_mass: 10.0,
            bob2_mass: 10.0,
            arm1_angular_position: 1.5 * PI,
            arm2_angular_position: 1.5 * PI,
            arm1_angular_velocity: 0.0,
            arm2_angular_velocity: 0.0,
        };

        let pendulum2 = Pendulum {
            arm1_length: 10.0,
            arm2_length: 10.0,
            bob1_mass: 10.0,
            bob2_mass: 10.0,
            arm1_angular_position: 0.5 * PI,
            arm2_angular_position: 0.5 * PI,
            arm1_angular_velocity: 0.0,
            arm2_angular_velocity: 0.0,
        };

        assert!(pendulum1.acc_arm2() - 0.0 < FLOAT_TOLERANCE);
        assert!(pendulum2.acc_arm2() - 0.0 < FLOAT_TOLERANCE);
    }
}
