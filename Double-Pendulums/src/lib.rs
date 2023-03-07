pub const G: f64 = 9.81;
use std::f64::consts::PI;

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
    pub fn scale(&self, k: f64) -> Vector2 {
        return Vector2 {
            x: k * self.x,
            y: k * self.y,
        };
    }

    pub fn add(&self, v: Vector2) -> Vector2 {
        return Vector2 {
            x: self.x + v.x,
            y: self.y + v.y,
        };
    }

    pub fn subtract(&self, v: Vector2) -> Vector2 {
        return Vector2 {
            x: self.x - v.x,
            y: self.y - v.y,
        };
    }

    pub fn dot(&self, v: Vector2) -> f64 {
        return self.x * v.x + self.y * v.y;
    }

    pub fn cross(&self, v: Vector2) -> f64 {
        // returns f64 despite being cross product as a two dimensional cross product has only one non zero term in three dimensions
        return self.x * v.y - self.y * v.x;
    }
}

pub struct PendulumState {
    // angular position in rad, +ve is counter-clockwise, vertically down is 0
    // angular velocity in rad/s, +vs is counter-clockwise
    pub arm1_angular_position: f64,
    pub arm1_angular_velocity: f64,

    pub arm2_angular_position: f64,
    pub arm2_angular_velocity: f64,
}

impl PendulumState {
    pub fn scale_state(&self, h: f64) -> PendulumState {
        // Increment arm1 state
        return PendulumState {
            arm1_angular_position: self.arm1_angular_position * h,
            arm1_angular_velocity: self.arm1_angular_velocity * h,
            arm2_angular_position: self.arm2_angular_position * h,
            arm2_angular_velocity: self.arm2_angular_velocity * h,
        };
    }

    pub fn add_state(&self, state: PendulumState) -> PendulumState {
        return PendulumState {
            arm1_angular_position: self.arm1_angular_position + state.arm1_angular_position,
            arm1_angular_velocity: self.arm1_angular_velocity + state.arm1_angular_velocity,
            arm2_angular_position: self.arm2_angular_position + state.arm2_angular_position,
            arm2_angular_velocity: self.arm2_angular_velocity + state.arm2_angular_velocity,
        };
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
    // computed at initialisation in new()
    // stores maximum possible potential energy
    // used to compute error as at rest, kinetic energy is zero
    // and pendulums are initialised at rest
    // since total energy = kinetic energy + potential energy, and kinetic is zero,
    // total energy = maximum potential energy
    pub potential_energy_max: f64,
}

impl Pendulum {
    pub fn new(
        bob1_mass: f64,
        bob2_mass: f64,
        arm1_length: f64,
        arm2_length: f64,
        arm1_angular_position: f64,
        arm2_angular_position: f64,
    ) -> Pendulum {
        let mut pendulum = Pendulum {
            bob1_mass: bob1_mass,
            bob2_mass: bob2_mass,
            arm1_length: arm1_length,
            arm2_length: arm2_length,
            arm1_angular_position: arm1_angular_position,
            arm2_angular_position: arm2_angular_position,
            arm1_angular_velocity: 0.0,
            arm2_angular_velocity: 0.0,
            potential_energy_max: 0.0,
        };
        pendulum.potential_energy_max = pendulum.energy_potential();

        return pendulum;
    }

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
        // Kinetic energy sources:
        // https://rotations.berkeley.edu/the-double-pendulum/
        // http://www.maths.surrey.ac.uk/explore/michaelspages/documentation/Double.pdf
        // https://www.phys.lsu.edu/faculty/gonzalez/Teaching/Phys7221/DoublePendulum.pdf
        let bob1_velocity = self.arm1_length * self.arm1_angular_velocity;
        let bob2_velocity = self.arm2_length * self.arm2_angular_velocity;
        let mut kinetic_energy = 0.0;
        kinetic_energy = kinetic_energy + 0.5 * self.bob1_mass * bob1_velocity.powi(2);
        kinetic_energy = kinetic_energy
            + 0.5
                * self.bob2_mass
                * (bob1_velocity.powi(2)
                    + bob2_velocity.powi(2)
                    + 2.0
                        * bob1_velocity
                        * bob2_velocity
                        * (self.arm1_angular_position - self.arm2_angular_position).cos());
        // let mut kinetic_energy = 0.0;
        // kinetic_energy = kinetic_energy
        //     + 0.5 * self.bob1_mass * self.arm1_length.powi(2) * self.arm1_angular_velocity.powi(2);
        // kinetic_energy = kinetic_energy
        //     + 0.5
        //         * self.bob2_mass
        //         * (self.arm2_length.powi(2) * self.arm2_angular_velocity.powi(2)
        //             + 2.0
        //                 * self.arm1_length
        //                 * self.arm2_length
        //                 * self.arm1_angular_velocity
        //                 * self.arm2_angular_velocity
        //                 * (self.arm1_angular_position - self.arm2_angular_position).cos()
        //             + self.arm1_length.powi(2) * self.arm1_angular_velocity.powi(2));

        return kinetic_energy;
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
        // Returns acceleration on arm1 from current pendulum state
        return self.acc_arm1_at_position(&PendulumState {
            arm1_angular_position: self.arm1_angular_position,
            arm1_angular_velocity: self.arm1_angular_velocity,
            arm2_angular_position: self.arm2_angular_position,
            arm2_angular_velocity: self.arm2_angular_velocity,
        });
    }

    pub fn acc_arm2(&self) -> f64 {
        // Returns acceleration on arm2 from current pendulum state
        return self.acc_arm2_at_position(&PendulumState {
            arm1_angular_position: self.arm1_angular_position,
            arm1_angular_velocity: self.arm1_angular_velocity,
            arm2_angular_position: self.arm2_angular_position,
            arm2_angular_velocity: self.arm2_angular_velocity,
        });
    }

    // acc_arm_at_position functions returns acceleration on arms from given position and velocity, and current arm length and mass
    pub fn acc_arm1_at_position(&self, state: &PendulumState) -> f64 {
        // Returns acceleration on arm1 from given position and velocity
        let term1 =
            -G * (2.0 * self.bob1_mass + self.bob2_mass) * (state.arm1_angular_position).sin();

        let term2 = -(self.bob2_mass * G)
            * (state.arm1_angular_position - 2.0 * state.arm2_angular_position).sin();

        let term3 = -2.0
            * (state.arm1_angular_position - state.arm2_angular_position).sin()
            * self.bob2_mass
            * (state.arm2_angular_velocity.powi(2) * self.arm2_length
                + state.arm1_angular_velocity.powi(2)
                    * self.arm1_length
                    * (state.arm1_angular_position - state.arm2_angular_position).cos());

        let denominator = self.arm1_length
            * (2.0 * self.bob1_mass + self.bob2_mass
                - self.bob2_mass
                    * (2.0 * state.arm1_angular_position - 2.0 * state.arm2_angular_position)
                        .cos());

        return (term1 + term2 + term3) / denominator;
    }

    pub fn acc_arm2_at_position(&self, state: &PendulumState) -> f64 {
        // Returns acceleration on arm2 from given position and velocity
        let term1 = 2.0 * (state.arm1_angular_position - state.arm2_angular_position).sin();

        let term2 = state.arm1_angular_velocity.powi(2)
            * self.arm1_length
            * (self.bob1_mass + self.bob2_mass);

        let term3 = G * (self.bob1_mass + self.bob2_mass) * (state.arm1_angular_position).cos();

        let term4 = state.arm2_angular_velocity.powi(2)
            * self.arm2_length
            * self.bob2_mass
            * (state.arm1_angular_position - state.arm2_angular_position).cos();

        let denominator = self.arm2_length
            * (2.0 * self.bob1_mass + self.bob2_mass
                - self.bob2_mass
                    * (2.0 * state.arm1_angular_position - 2.0 * state.arm2_angular_position)
                        .cos());

        return (term1 * (term2 + term3 + term4)) / denominator;
    }

    pub fn clean_angles(&mut self) {
        // Keeps angles between -2 * PI to 2 * PI
        if self.arm1_angular_position > 2.0 * PI {
            self.arm1_angular_position = self.arm1_angular_position - 2.0 * PI;
        } else if self.arm1_angular_position < -2.0 * PI {
            self.arm1_angular_position = self.arm1_angular_position + 2.0 * PI;
        }

        if self.arm2_angular_position > 2.0 * PI {
            self.arm2_angular_position = self.arm2_angular_position - 2.0 * PI;
        } else if self.arm2_angular_position < -2.0 * PI {
            self.arm2_angular_position = self.arm2_angular_position + 2.0 * PI;
        }
    }

    pub fn update(&mut self, t0: f64, t1: f64, h: f64, method: &str) {
        let mut t = t0;
        while t < t1 {
            if method == "euler" {
                self.update_euler(t, h);
            } else if method == "semi_implicit_euler" {
                self.update_semi_implicit_euler(t, h);
            } else if method == "rk4" {
                self.update_rk4(t, h);
            } else {
                panic!("Invalid integrator option");
            }
            t = t + h;
        }
    }

    pub fn update_euler(&mut self, t: f64, h: f64) {
        let new_arm1_angular_velocity = self.arm1_angular_velocity + h * self.acc_arm1();
        let new_arm2_angular_velocity = self.arm2_angular_velocity + h * self.acc_arm2();
        let new_arm1_angular_position = self.arm1_angular_position + h * self.arm1_angular_velocity;
        let new_arm2_angular_position = self.arm2_angular_position + h * self.arm2_angular_velocity;

        self.arm1_angular_velocity = new_arm1_angular_velocity;
        self.arm2_angular_velocity = new_arm2_angular_velocity;
        self.arm1_angular_position = new_arm1_angular_position;
        self.arm2_angular_position = new_arm2_angular_position;
        self.clean_angles();
    }

    pub fn update_semi_implicit_euler(&mut self, t: f64, h: f64) {
        let new_arm1_angular_velocity = self.arm1_angular_velocity + h * self.acc_arm1();
        let new_arm2_angular_velocity = self.arm2_angular_velocity + h * self.acc_arm2();
        self.arm1_angular_velocity = new_arm1_angular_velocity;
        self.arm2_angular_velocity = new_arm2_angular_velocity;

        let new_arm1_angular_position = self.arm1_angular_position + h * self.arm1_angular_velocity;
        let new_arm2_angular_position = self.arm2_angular_position + h * self.arm2_angular_velocity;
        self.arm1_angular_position = new_arm1_angular_position;
        self.arm2_angular_position = new_arm2_angular_position;

        self.clean_angles();
    }

    pub fn rk4_rhs(&self, t: f64, state: &PendulumState) -> PendulumState {
        let output = PendulumState {
            arm1_angular_position: state.arm1_angular_velocity,
            arm1_angular_velocity: self.acc_arm1_at_position(state),
            arm2_angular_position: state.arm2_angular_velocity,
            arm2_angular_velocity: self.acc_arm2_at_position(state),
        };
        return output;
    }

    pub fn update_rk4(&mut self, t: f64, h: f64) {
        let state = PendulumState {
            arm1_angular_position: self.arm1_angular_position,
            arm1_angular_velocity: self.arm1_angular_velocity,
            arm2_angular_position: self.arm2_angular_position,
            arm2_angular_velocity: self.arm2_angular_velocity,
        };

        let k1 = self.rk4_rhs(t, &state);
        let k2 = self.rk4_rhs(t + 0.5 * h, &state.add_state(k1.scale_state(0.5 * h)));
        let k3 = self.rk4_rhs(t + 0.5 * h, &state.add_state(k2.scale_state(0.5 * h)));
        let k4 = self.rk4_rhs(t + h, &state.add_state(k3.scale_state(h)));

        self.arm1_angular_position = self.arm1_angular_position
            + (h / 6.0)
                * (k1.arm1_angular_position
                    + 2.0 * k2.arm1_angular_position
                    + 2.0 * k3.arm1_angular_position
                    + k4.arm1_angular_position);
        self.arm1_angular_velocity = self.arm1_angular_velocity
            + (h / 6.0)
                * (k1.arm1_angular_velocity
                    + 2.0 * k2.arm1_angular_velocity
                    + 2.0 * k3.arm1_angular_velocity
                    + k4.arm1_angular_velocity);

        self.arm2_angular_position = self.arm2_angular_position
            + (h / 6.0)
                * (k1.arm2_angular_position
                    + 2.0 * k2.arm2_angular_position
                    + 2.0 * k3.arm2_angular_position
                    + k4.arm2_angular_position);
        self.arm2_angular_velocity = self.arm2_angular_velocity
            + (h / 6.0)
                * (k1.arm2_angular_velocity
                    + 2.0 * k2.arm2_angular_velocity
                    + 2.0 * k3.arm2_angular_velocity
                    + k4.arm2_angular_velocity);

        self.clean_angles();
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
            arm1_angular_position: 0.0,
            arm2_angular_position: 0.0,
            ..Default::default()
        };

        let pendulum2 = Pendulum {
            arm1_length: 10.0,
            arm2_length: 10.0,
            bob1_mass: 10.0,
            bob2_mass: 10.0,
            arm1_angular_velocity: -1.0,
            arm2_angular_velocity: 0.0,
            arm1_angular_position: 0.0,
            arm2_angular_position: 0.0,
            ..Default::default()
        };

        assert!((pendulum1.energy_kinetic() - 2500.0).abs() < FLOAT_TOLERANCE);
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
    fn test_potential_energy_max() {
        let pendulum1 = Pendulum::new(10.0, 10.0, 10.0, 10.0, 1.0 * PI, 1.0 * PI);
        let pendulum2 = Pendulum::new(5.0, 15.0, 10.0, 10.0, 0.0 * PI, 0.0 * PI);

        assert!((pendulum1.potential_energy_max - 6867.0).abs() < FLOAT_TOLERANCE);
        assert!((pendulum2.potential_energy_max - 490.5).abs() < FLOAT_TOLERANCE);
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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
        };

        // This particular state arises from the Jupyter Notebook implementation
        // RK4 with h = 0.01, on the 1000th index
        let pendulum3 = Pendulum {
            arm1_length: 100.0,
            arm2_length: 100.0,
            bob1_mass: 10.0,
            bob2_mass: 10.0,
            arm1_angular_position: 3.1424357386212893,
            arm2_angular_position: 3.140478722190667,
            arm1_angular_velocity: 0.00047841035932873357,
            arm2_angular_velocity: -0.0006580958030288573,
            ..Default::default()
        };
        println!("{}", pendulum3.acc_arm2());
        assert!((pendulum1.acc_arm2() - 0.0).abs() < FLOAT_TOLERANCE);
        assert!((pendulum2.acc_arm2() - 0.0).abs() < FLOAT_TOLERANCE);
        assert!((pendulum3.acc_arm2() - -0.00038396302819984884).abs() < FLOAT_TOLERANCE);
    }
}
