mod args;
mod variable;

use args::Args;
use clap::Parser;
use variable::{save_trajectory, Variables};


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

    save_trajectory(trajectory);

}


