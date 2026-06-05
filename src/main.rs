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
    let method_is_euler = false;
    for _t in 0..args.simsteps {
        let derivative = v.derivative(sigma, rho, beta);
        if method_is_euler
        {
            let xnew = v.x + args.dt * derivative[0];
            let ynew = v.y + args.dt * derivative[1];
            let znew = v.z + args.dt * derivative[2];


            let vnew = Variables{x:xnew, y:ynew, z:znew};
            trajectory.push(vnew.clone());
            v = vnew;
        }
        else {

            let xmid = v.x + args.dt * 0.5 * derivative[0];
            let ymid = v.y + args.dt * 0.5 * derivative[1];
            let zmid = v.z + args.dt * 0.5 * derivative[2];

            let vmid = Variables{x:xmid, y:ymid, z:zmid};
            let mid_derivative = vmid.derivative(sigma, rho, beta);

            let xfinal = v.x + args.dt * mid_derivative[0];
            let yfinal = v.y + args.dt * mid_derivative[1];
            let zfinal = v.z + args.dt * mid_derivative[2];

            let finalmid = Variables{x:xfinal, y:yfinal, z:zfinal};
            v = finalmid;
            trajectory.push(v.clone());
        }


    }

    save_trajectory(trajectory);

}


