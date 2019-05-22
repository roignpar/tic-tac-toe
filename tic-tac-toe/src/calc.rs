use quicksilver::geom::Vector;

pub fn midpoint<V: Into<Vector>>(v1: V, v2: V) -> Vector {
    let (p1, p2) = (v1.into(), v2.into());

    ((p1.x + p2.x) / 2.0, (p1.y + p2.y) / 2.0).into()
}

pub fn inside_rectangle(top_left: Vector, bottom_right: Vector, point: Vector) -> bool {
    point.x > top_left.x
        && point.y > top_left.y
        && point.x < bottom_right.x
        && point.y < bottom_right.y
}
