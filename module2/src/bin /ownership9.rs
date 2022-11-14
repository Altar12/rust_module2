fn main() {
    let x = Box::new(5);

    let mut y = x.clone(); // Implement this line, dont change other lines!

    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}
