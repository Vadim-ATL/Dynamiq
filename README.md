<p align="center">
  <a href="" rel="noopener">
 <img width=400 height=300 src="https://i.imgur.com/NArMu5J.png" alt="DynamiQ"></a>
</p>

<h3 align="center">DynamiQ</h3>

<div align="center">

[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![GitHub Issues](https://img.shields.io/github/issues/Vadim-ATL/Dynamiq.svg?cacheSeconds=60)](https://github.com/Vadim-ATL/Dynamiq/issues)
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/Vadim-ATL/Dynamiq.svg?cacheSeconds=60)](https://github.com/Vadim-ATL/Dynamiq/pulls)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](/LICENSE)

</div>

---

<p align="center"> DynamiQ is a powerful package built on Rust for effective, simple solving of ODE/PDE equations.
    <br> 
</p>

## Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Tests](#tests)
- [Built Using](#built_using)
- [TODO](TODO.md)
- [LICENSE](LICENSE.md)
- [Authors](#authors)

## About <a name = "about"></a>

DynamiQ is a powerful numerical solver built on Rust for effective, simple solving of Ordinary Differential Equations (ODEs) and Partial Differential Equations (PDEs). The project includes multiple popular scientific methods for solving differential equations:

- **Runge-Kutta (RK4)**
- **Euler’s Method** (planned)
- **Adams-Bashforth** (planned)
- **Finite Difference Methods (FDM)** (planned)
- **Spectral Methods** (planned)

## Getting Started <a name = "getting_started"></a>

Make sure you have **Rust** installed with all dependencies:  
- [Install Rust](https://www.rust-lang.org/tools/install)
```
cargo add ndarray
cargo add num
```

### Dependencies

| Dependency  | Version  | Features |
|------------|----------|----------|
| `ndarray`  | `0.16.1` | -        |
| `num`      | `0.4.3`  | -        |


### Installing

### Clone the repository:
```
git clone https://github.com/yourusername/DynamiQ.git
cd DynamiQ
```

Define a differential equation in main.rs:
```
pub struct HarmonicOscillator<T: Float> {
    omega: T,
}

impl<T: Float> HarmonicOscillator<T> {
    pub fn new(omega: T) -> Self {
        Self { omega }
    }
}

impl<T: Float> DifferentialEquation<T> for HarmonicOscillator<T> {
    fn evaluate(&self, _t: T, state: &ArrayView1<T>, derivatives: &mut Array1<T>) {
        derivatives[0] = state[1]; // dx/dt = v
        derivatives[1] = -self.omega * self.omega * state[0]; // dv/dt = -ω²x
    }

    fn dimension(&self) -> usize {
        2
    }

    /// Exact solution: x(t) = cos(ωt), v(t) = -sin(ωt)
    fn exact_solution(&self, t: T) -> Option<Array1<T>> {
        let time = t.to_f64()?; 
        let omega = self.omega.to_f64()?; 
        let exact_x = T::from((omega * time).cos())?;
        let exact_v = T::from(-(omega * time).sin())?;
        
        Some(Array1::from(vec![exact_x, exact_v]))
    }
}
```

Create the integrator and equation objects. Ensure that the solver object is mutable.

```
let omega: f64 = 2.0 * PI;  

let oscillator = HarmonicOscillator { omega };    
let integrator = Box::new(RK4::RK4_Method::new(2));
let mut solver: ODESolver<f64> = ODESolver::new(0.01, 2.0, Array1::from(vec![1.0, 0.0]), integrator);
```

Run the Rust project

```
cargo run
```

The output will be in format

```
First values (t, numerical, exact, error):
t = 0.000: 1.000000, 1.000000, 0.000000e0
t = 0.010: 0.998027, 0.998027, 8.545076e-11
t = 0.020: 0.992115, 0.992115, 1.195275e-9
t = 0.030: 0.982287, 0.982287, 3.322397e-9
t = 0.040: 0.968583, 0.968583, 6.451693e-9
[MAE] Maximum absolute error: 1.4405e-6
```

## Running the tests <a name = "tests"></a>

The tests will be added in further commits

## Built Using <a name = "built_using"></a>

- [Rust](https://www.rust-lang.org/tools/install) - Rust for fast numerical computations
- [ndarray](https://docs.rs/ndarray/latest/ndarray/) - Numerical arrays
- [num](https://docs.rs/num/latest/num/) - Mathematical utilities

## Contributing

Pull requests are always welcome 🤗, I am starting this project, so feel free to suggest, criticize (hope constructively).

For major changes, please open an issue first to discuss what you would like to change.

## Authors <a name = "authors"></a>

- [@Vadim-ATL](https://github.com/Vadim-ATL) - Project Idea & Initial work
- [Me](https://github.com/Vadim-ATL) - Algorithm, Traits, Patterns Design
- [Well, also me](https://github.com/Vadim-ATL) - Python Bindings & pyo3 Integration
- [Guess what 👀](https://github.com/Vadim-ATL) - Testing & Documentation

