use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The Prandtl number
    #[arg(short, long, default_value_t = 10.0)]
    sigma: f64,
    /// The Rayleigh number
    #[arg(short, long, default_value_t = 20.0)]
    rho: f64,
    /// The dimensions of fluid layer number
    #[arg(short, long, default_value_t = 8.0/3.0)]
    beta: f64,

}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
