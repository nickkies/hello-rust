#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn react_area(rect: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rect;

    ((x1 - x2) * (y1 - y2)).abs()
}

fn square(point: Point, f: f32) -> Rectangle {
    Rectangle {
        top_left: point,
        bottom_right: Point { x: f, y: f },
    }
}

fn structures() {
    let name = String::from("Nick");
    let age = 30;
    let peter = Person { name, age };
    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: left_edge, y: right_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: right_edge },
        bottom_right,
    };

    // let Rectangle { top_left: point1, bottom_right: Point { x: x2, y: y2} } = _rectangle;
    // let Point { x: x1, y: y1 } = point1;
    // println!("Rectangle destructuring: {}, {}, {}, {}", x1, y1, x2, y2);

    let area = react_area(_rectangle);
    println!("react_area: {}", area);

    let _square = square(Point { x: 1.1, y: 1.1 }, 1.5);
    println!("square: {:?}", _square);

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
