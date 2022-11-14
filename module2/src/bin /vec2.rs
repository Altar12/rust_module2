// use a hashmap to store String as key and i32 as value
// use a for loop to print the data in hashmap

// try inserting elements into hashmap without reference
// see what compiler error you get!

use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();
    marks.insert("Alex".to_string(), 50);
    marks.insert("Brandon".to_string(), 40);
    marks.insert("Catherine".to_string(), 46);
    for (name, marks) in marks.iter() {
        println!("{} scored {} marks", name, marks);
    }
    let alex_marks = marks.get(&"Alex".to_string()); //get requires a ref to be passed
    match alex_marks {
        Some(marks) => println!("Alex scored {} marks", marks),
        None => println!("Could not find entry corresponding to Alex"),
    }
}
