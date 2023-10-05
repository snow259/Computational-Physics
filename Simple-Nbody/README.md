# Simple-Nbody

This is an implementation of an n body gravity simulation. This particular implementation computes all accelerations between all masses, leading to n^2 computations per time step and a time complexity of O(n^2).

Currently only implements the Euler integrator, with other options being considered for future implementation.

Not many simulation parameters can currently be changed, but futher options are planned.

## Simulation Options

Currently, two places can be easily edited to change some simulation parameters:

###

Navigating to this line here:

```rust
    let n = 500;
```

n is the number of bodies in the simulation. Larger numbers are not a good idea as the time complexity rises with n^2. On my system (i5 9300H @ 2.4GHz), more than 500 bodies starts showing noticable slowdowns. A future implementation of the Barnes-Hut algorithm should improve performance dramatically.

### Screenshots

Changing the

```rust
pub const SAVE_SCREENSHOT: bool = false;
```

constant to true will make the simulation save each frame to disk. First, a folder named "screenshots" must be created in the same directory. Files are saved with the name of "[iteration number].png". These screenshots can then be animated into a video using ffmpeg.

## While Running

Nothing can be changed while the simulation is running. Screenshots will be written to disk if the option mentioned above is set to true.