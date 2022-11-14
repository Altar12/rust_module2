// Use a enum with variants as different car names and
//  create a function which accepts an &str,
// using match statement return the correct enum variant
// define a struct Details
// car:Car
//price:i32
#[derive(Debug)]
enum Car {
    Ferrari,
    Bughati,
    Porsche,
}
fn create_car(name: &str) -> Car {
    let name = &name.to_lowercase()[..];
    match name {
        "ferrari" => Car::Ferrari,
        "bughati" => Car::Bughati,
        "porsche" => Car::Porsche,
        _ => panic!("Invalid car name passed"),
    }
}
#[derive(Debug)]
struct Details {
    car: Car,
    price: i32,
}
fn main() {
    let my_car = Details {
        car: create_car("ferrari"),
        price: 50000,
    };
    println!("{:?}", my_car);
}
