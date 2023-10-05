# Double-Pendulums

This is an implementation of the chaotic system of double pendulums. A handful of options to define pendulums and numerical integrators are currently implemented. Current major limitation form a user perspective is changing these options currently requires modifying the source code and recompiling. A GUI or other form of menu is currently on the todo list.

## Integrators

Currently, four integrators are implemented:

- Euler
- Semi implicit Euler
- Runge-Kutta 4th order
- Leapfrog

To change integrators, navigate to the following section of code in "main.rs":

```rust
        if paused == false {
            for pendulum in &mut pendulums {
                pendulum.update(t0, t0 + frame_time_increment, h, method0);
            }
```

And change method0 to any of the other methods implemented. The mapping is as follows:

```rust
    let method0 = "euler";
    let method1 = "semi_implicit_euler";
    let method2 = "rk4";
    let method3 = "leap_frog";
```

## Pendulum Options

Pendulum options are all set by changing the function arguments to the call that generates the pendulums before the simulation loop:

```rust
    let mut pendulums = new_pendulums(n_pendulums, arm1_offset, arm2_offset, initial_arm1_fraction, arm1_fraction_offset, inital_total_length, total_length_increment);
```

- n_pendulums: Number of pendulums
- arm1_offset: Each successive pendulum will have its first arm position incremented by this amount
- arm2_offset: Each successive pendulum will have its second arm position incremented by this amount
- initial_arm1_fraction: Length of the first arm as a fraction of the total length (0.5 would mean half the total length is the first arm)
- arm1_fraction_offset: Each successive pendulum will have the length of the first arm changed as its fraction changes
- initial_total_length: Total length of the first pendulum (lengths of first and second arms added together)
- total_length_increment: Each successive pendulum will have its total length incremented by this amount

## While Running

- Hit space to toggle pause
- Energy in the system is displayed on the top left
- FPS is displayed on the top right
