# ODE-Solver-Comparison

A comparison of the performance of different numerical methods of solving a system of ordinary differential equations. Highly in development at the moment, goals include:

- Comparing errors
- Comparing computational efficiency
- Gain familiarity with implementing solvers for future work in Rust

## Testing

Unit tests are implemented for all solvers. The test involves passing a function and the differential of that function, along with necessary arguments for the solver. The output of the solver is compared to the original function within a certain tolerance to verify that it is functioning as expected.

With the package odetest installed in development mode, tests can be run with:

```
	pytest odetest
```