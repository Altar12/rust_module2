// Topic: Getting Used to with struct
//
//Requirements:
/*
 1-Define  struct Shape having  square:Square and rectangle:Rectangle
 2-Square has one field as side:i32
 3-Rectangle has two length:i32 and width:i32
 4-Create a function which returns a new Rectangle
 5-Create a function which returns a new Square
 6-Create a function which returns a new Shape
 7-Create a function which takes Shape and prints the dimension of all shapes
*/

struct Square {
    side: i32,
}
struct Rectangle {
    length: i32,
    width: i32,
}
struct TwoShapes {
    square: Square,
    rectangle: Rectangle,
}
fn square(side: i32) -> Square {
    Square { side }
}
fn rectangle(length: i32, width: i32) -> Rectangle {
    Rectangle { length, width }
}
fn shape(square: Square, rectangle: Rectangle) -> TwoShapes {
    TwoShapes { square, rectangle }
}
fn print_shapes(shapes: &TwoShapes) {
    println!("A square of side {}", shapes.square.side);
    println!(
        "A rectangle of dimension {}X{}",
        shapes.rectangle.length, shapes.rectangle.width
    );
}

fn main() {
    let shapes = shape(square(12), rectangle(20, 15));
    print_shapes(&shapes);
}
