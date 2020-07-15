struct Point {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32 = 0.)
    {
        self.w = 1.;
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn to_array() -> [f32, 4]
    {
        return [ self.x, self.y, self.z, self.w ];
    }
}