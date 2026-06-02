
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