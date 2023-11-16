use orphan_rule::Point;

struct PointWrapper(Point);

impl PartialEq for PointWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.get_x() == other.0.get_x() && self.0.get_y() == other.0.get_y()
    }
}

fn main() {
    let p1 = PointWrapper(Point::new(1, 2));
    let p2 = PointWrapper(Point::new(1, 2));

    println!("{}", p1 == p2);
}
