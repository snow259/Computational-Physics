const G: f32 = 9.81;

pub struct Coordinates(pub f32, pub f32);

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
    pub arm1_angular_velocity: f64,
    pub arm2_angular_velocity: f64,
}

impl Pendulum {
    pub fn bob_coordinates(&self) -> [Coordinates; 2] {
        let bob1_x = self.arm1_length * self.arm1_angular_position.sin();
        let bob1_y = self.arm1_length * self.arm1_angular_position.cos();
        let bob1_coordinates = Coordinates(bob1_x, bob1_y);

        let bob2_x = bob1_x + self.arm2_length * self.arm2_angular_position.sin();
        let bob2_y = bob1_y + self.arm2_length * self.arm2_angular_position.cos();
        let bob2_coordinates = Coordinates(bob2_x, bob2_y);

        return [bob1_coordinates, bob2_coordinates];
    }

    pub fn energy_kinetic(&self) -> f32 {
        0.0
    }

    pub fn energy_potential(&self) -> f32 {
        let height_pendulum_base = self.arm1_length + self.arm2_length;
        let bob_coordinates = self.bob_coordinates();
        let Coordinates(x1, y1) = bob_coordinates[0];
        let Coordinates(x2, y2) = bob_coordinates[1];

        let potential_energy = self.bob1_mass * (height_pendulum_base + y1)
            + self.bob2_mass * (height_pendulum_base + y2);

        return potential_energy * G;
    }

    pub fn energy_total(&self) -> f32 {
        let total_energy = self.energy_potential() + self.energy_kinetic();

        return total_energy;
    }
}
