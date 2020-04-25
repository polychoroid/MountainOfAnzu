// Quadrilaterals have the advantage of being their own bounding boxes ;)
// The boundry box has verticies aligned to this convention
//
// a -- b
// |    |
// c -- d
//
// x is horizontal 
// y is vertical
// z in right handed cross product of x and y
// w is maintented for rotations.  

struct Rectangle {
    position: Point,
    width: f32,
    height: f32
}

impl Rectangle {
    pub fn new(position: Point, width: f32, height: f32) -> Self
    {
        self.position = position;
        self.width = width;
        self.height = height;
    }
}

impl Shape for Rectangle {
   pub fn position() -> Point {
        return self.position;
   }

   pub fn area() -> f32 {
        return width * height;
   }

   pub fn boundry() -> Vec<Point> {
    let a = Point::new(self.position.x - (.5 * width), self.position.y + (.5 * height));
    let b = Point::new(self.position.x + (.5 * width), self.position.y + (.5 * height));    
    let c = Point::new(self.position.x - (.5 * width), self.position.y - (.5 * height));
    let d = Point::new(self.position.x + (.5 * width), self.position.y - (.5 * height));
    return vec![a, b, c, d];
   }
}

struct Square {
    position: Point,
    side: f32
}

impl Square {
    pub fn new(position: Point, side: f32) -> Self
    {
        self.position = position;
        self.side = side;
    }
}

impl Shape for Square {
   pub fn position() -> Point {
        return self.position;
   }

   pub fn area() -> f32 {
        return side * side;
   }

   pub fn boundry() -> Vec<Point> {
    let a = Point::new(self.position.x - (.5 * side), self.position.y + (.5 * side));
    let b = Point::new(self.position.x + (.5 * side), self.position.y + (.5 * side));    
    let c = Point::new(self.position.x - (.5 * side), self.position.y - (.5 * side));
    let d = Point::new(self.position.x + (.5 * side), self.position.y - (.5 * side));
    return vec![a, b, c, d];
   }
}