
//structs

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

//pub struct Gun {
//    pub fire: bool,
//    pub reload_time: f64,
//    pub position: Point,
//}

pub struct Ship {
    pub position: Point,
    pub speed: Point,
    pub accel: Point,
    pub rotation: f64,
    pub rot_speed: f64,
    pub rot_accel: f64,
}

//pub struct Projectile {
//    pub position: Point,
//    pub speed: Point,
//    pub rotation: f64,
//    pub lifespan: f64,
//}
//
//pub struct Star {
//    pub position: Point,
//    pub speed: Point,
//    pub rotation: f64,
//}
