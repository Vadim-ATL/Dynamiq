### In Progress

Work on the other ODE/PDE methods @Vadim-ATL till 15.02

- [ ] Implement Euler’s Method for ODE solving
- [ ] Implement Adams-Bashforth Method for higher-order accuracy
- [ ] Implement Finite Difference Methods (FDM) for PDE solving
- [ ] Implement Spectral Methods for high-precision PDE solutions

### TODO

Performance & Optimization @Vadim-ATL till 01.03 

- [ ] Optimize Runge-Kutta (RK4) method for better performance
- [ ] Add parallelization (Rayon or OpenMP) for faster computations

Code Improvements @Vadim-ATL deadline - TBD

- [ ] Refactor solver code for better modularity
- [ ] Add unit tests for each solver method
- [ ] Add better error handling and logging
 
Bindings & API - @Vadim-ATL till 15.03

- [ ] Develop Python bindings using pyo3 for usability in Python
- [ ] Add a CLI tool for quick equation solving via command line

Testing & Validation - @Vadim-ATL till 15.03 

- [ ] Implement benchmark tests against SciPy solvers
- [ ] Compare accuracy of methods with analytical solutions
- [ ] Add regression tests to prevent breaking changes

### Done ✓

- [x] Make ODESolver class for better modularity
- [x] Add RK4 module for solving ODE
- [x] Move RK4 method for separate module
- [x] Add separate solution in ODESolver for numerical, exact solutions if equation has it
- [x] Add README.MD
