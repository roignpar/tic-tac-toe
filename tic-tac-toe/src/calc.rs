use quicksilver::geom::Vector;

pub type RawPoint = (f32, f32);

pub fn midpoint<V: Into<Vector>>(v1: V, v2: V) -> RawPoint {
    let (p1, p2) = (v1.into(), v2.into());

    ((p1.x + p2.x) / 2.0, (p1.y + p2.y) / 2.0)
}
