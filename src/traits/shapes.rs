trait Shape {
    fn position() -> Point;
    fn area() -> f32;
    fn boundry() -> Vec<Point>; // This should be considered to be the axis aligned bounding box for any shape.
}

trait Quadrilateral {
    fn box() -> [f32; 8];
}

trait Prisim {
    fn box() -> [f32; 16];
}