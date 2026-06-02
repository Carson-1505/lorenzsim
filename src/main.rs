mod args;

use args::Args;
use clap::Parser;

#[derive(Debug, Clone)]
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

    let mut v = Variables {x: 1.0, y: 1.0, z: 1.0};
    let mut trajectory = Vec::new();
    trajectory.push(v.clone());


    // Create for loop for each time step?
    for _t in 0..args.simsteps {
        let derivative = v.derivative(sigma, rho, beta);
        let xnew = v.x + args.dt*derivative[0];
        let ynew = v.y + args.dt*derivative[1];
        let znew = v.z + args.dt*derivative[2];


        let vnew = Variables{x:xnew, y:ynew, z:znew};
        trajectory.push(vnew.clone());
        v = vnew;

    }

    let mut wtr = csv::Writer::from_path("lorenzdata.csv").unwrap();
    wtr.write_record(["x", "y", "z"]).unwrap();

    for v in trajectory {
        wtr.write_record([v.x.to_string(), v.y.to_string(), v.z.to_string()]).unwrap();
    }

    wtr.flush().unwrap();

}
