use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The Prandtl number
    #[arg(short, long, default_value_t = 10.0)]
    pub sigma: f64,
    /// The Rayleigh number
    #[arg(short, long, default_value_t = 28.0)]
    pub rho: f64,
    /// The dimensions of fluid layer number
    #[arg(short, long, default_value_t = 8.0/3.0)]
    pub beta: f64,
    /// Time step (can also be 0.001)
    #[arg(short, long, default_value_t = 0.01)]
    pub dt: f64,
    /// Num of Simlation steps
    #[arg(short = 't', long, default_value_t = 10000)]
    pub simsteps: u32,
    /// User control between midpoint and Euler methods
    #[arg(short, long, default_value_t = true)]
    pub method_is_euler: bool,
}
