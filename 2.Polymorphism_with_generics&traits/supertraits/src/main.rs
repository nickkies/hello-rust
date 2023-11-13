trait Vehicle: Paint {
    fn park(&self);
    fn get_default_color() -> String {
        "black".to_owned()
    }
}

trait Paint {
    fn paint(&self, color: String) {
        println!("painting object: {color}");
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16,
}

struct Car {
    info: VehicleInfo,
}

impl Vehicle for Car {
    fn park(&self) {
        println!("parking car");
    }
}

impl Paint for Car {}

struct Truck {
    info: VehicleInfo,
}

impl Truck {
    fn unload(&self) {
        println!("unloading truck.");
    }
}

impl Vehicle for Truck {
    fn park(&self) {
        println!("parking truck");
    }
}

impl Paint for Truck {}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("painting house: {color}");
    }
}

fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "BMW".to_owned(),
            model: "X3".to_owned(),
            year: 2022,
        },
    };
    let house = House {};
    let object = create_paintable_object();

    paint_red(&car);
    paint_blue(&house);
    paint_red(&object);

    paint_vehicle_red(&car);
}

fn paint_red<T: Paint>(object: &T) {
    object.paint("red".to_owned());
}

fn paint_blue(object: &impl Paint) {
    object.paint("blue".to_owned());
}

fn paint_vehicle_red<T>(object: &T)
where
    T: Vehicle,
{
    object.paint("red".to_owned());
}

fn create_paintable_object() -> impl Paint {
    House {}
}
