enum QuadtreeState {
    Undivided,
    Divided
}


struct Quadtree<TMember> {
    I: Quadtree<TMember>,
    II: Quadtree<TMember>,
    III: Quadtree<TMember>,
    IV: Quadtree<TMember>,
    members: Vec<TMember>,,
    capacity: u8,    
}

impl Quadtree<TMember> where TMember: Shape {
    pub fn new() {

    }

    pub fn insert<TMember>(member: TMember) {

    }

    pub fn query() {

    }

    pub fn remove() {

    }

    pub fn update() {

    }

    fn subdivide() {

    }
}