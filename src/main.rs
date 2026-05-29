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
    /// Time step (can also be 0.001)
    #[arg(short, long, default_value_t = 0.01)]
    dt: f64,
    /// Num of Simlation steps
    #[arg(short = 't', long, default_value_t = 10000)]
    simsteps: u32,
}

    // Define x, y, z

#[derive(Debug)]
struct Variables {

    x: f64,
    y: f64,
    z: f64,
}

impl Variables {
    fn derivative(&self, sigma: f64, rho: f64, beta: f64) -> [f64; 3] {
        [
            sigma*( self.y - self.x ),
            (self.x*( rho - self.z)) - self.y,
            (self.x*self.y) - (beta*self.z),
        ]
    }
}
    // Create derivative Function for x, y, z

fn main() {
    let args = Args::parse();
    let sigma = args.sigma;
    let rho = args.rho;
    let beta = args.beta;

    println!("{args:?}");

    let v = Variables {x: 1.0, y: 1.0, z: 1.0};
    let derivative = v.derivative(sigma, rho, beta);

    println!("{v:?}");
    println!("{derivative:?}");

    // Create for loop for each time step?
   // for t in 0..args.simsteps {

    //}
}
