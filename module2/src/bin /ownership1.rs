// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter
struct Item {
    id: i32,
    quantity: i32,
}
fn print_id(item: &Item) {
    println!("Id: {}", item.id);
}
fn print_quantity(item: &Item) {
    println!("Quantity: {}", item.quantity);
}
fn main() {
    let item = Item {
        id: 3,
        quantity: 202,
    };
    let ref item_ref = item;
    print_id(item_ref);
    print_quantity(item_ref);
}
