
#[derive(Debug, Clone)]
pub struct Variables {

    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Variables {
    pub fn derivative(&self, sigma: f64, rho: f64, beta: f64) -> [f64; 3] {
        [
            sigma*( self.y - self.x ),
            (self.x*( rho - self.z)) - self.y,
            (self.x*self.y) - (beta*self.z),
        ]
    }
}

pub fn save_trajectory(trajectory: Vec<Variables>) {
    let mut wtr = csv::Writer::from_path("lorenzdata.csv").unwrap();
    wtr.write_record(["x", "y", "z"]).unwrap();

    for v in trajectory {
        wtr.write_record([v.x.to_string(), v.y.to_string(), v.z.to_string()]).unwrap();
    }

    wtr.flush().unwrap();
}
