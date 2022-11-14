// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
enum Color {
    Black,
    Blue,
    Yellow,
    Brown,
}
fn print_clr(color: &Color) {
    let clr_name = match color {
        Color::Black => "Black",
        Color::Blue => "Blue",
        Color::Yellow => "Yellow",
        Color::Brown => "Brown",
    };
    println!("{clr_name}");
}
fn main() {
    let my_clr = Color::Blue;
    print_clr(&my_clr);
}
