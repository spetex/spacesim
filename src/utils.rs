use celestial::Celestial;
use constants::GRAVITATIONAL_CONSTANT;

pub fn get_distance(a: &Celestial, b: &Celestial) -> f64 {
    let squared_x: f64 =
        (a.coordinates[0] - b.coordinates[0]) * (a.coordinates[0] - b.coordinates[0]);
    let squared_y: f64 =
        (a.coordinates[1] - b.coordinates[1]) * (a.coordinates[1] - b.coordinates[1]);
    let squared_z: f64 =
        (a.coordinates[2] - b.coordinates[2]) * (a.coordinates[2] - b.coordinates[2]);
    let sum: f64 = squared_x + squared_y + squared_z;
    sum.sqrt()
}

pub fn get_pull_force(mass1: f64, mass2: f64, coord1: f64, coord2: f64, distance: f64) -> f64 {
    GRAVITATIONAL_CONSTANT * ((mass1 * mass2) * (coord2 - coord1) / distance.powf(3.0))
}
