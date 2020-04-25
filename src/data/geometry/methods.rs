enum Intersect {
    Inside,
    Outside
}

pub fun 2d_in_boundry<Shape>(shape: Shape, point: Point) -> Intersect
{
    let mut minX = 0.;
    let mut maxX = 0.;
    let mut minY = 0.;
    let mut maxY = 0.;
    
    for point in shape.boundry() {
        if (point.x > maxX) {
            maxX = point.x;
        }

        if (point.x < minX) {
            minX = point.x;
        }

        if (point.y > maxY) {
            maxY = point.y;
        }

        if (point.y < minY) {
            minY = point.y;
        }
    }

    if (point.x < minX || p.x > maxX || p.y < minY || p.y > maxY) {
        return Outside;
    }

    return Inside;
}