//Topic:Working with enum
//
//

/*
1-Define a Enum Phones with some brands of phones,
2-define an enum Headsets with some brands  ,
3-define a struct Shop having two fields Phones and Headsets,
4-create instance of Shop and print which phone and headsets you have picked
*/
enum Phones {
    Samsung,
    Apple,
    Xiaomi,
    Poco,
}
enum Headsets {
    Boat,
    Jabra,
    Xbox,
}
struct Shop {
    phone: Phones,
    headsets: Headsets,
}

fn main() {
    let shop = Shop {
        phone: Phones::Apple,
        headsets: Headsets::Boat,
    };
    match shop.phone {
        Phones::Samsung => println!("I got samsung phone"),
        Phones::Apple => println!("I got apple phone"),
        Phones::Xiaomi => println!("I got xiaomi phone"),
        Phones::Poco => println!("I got poco phone"),
    }
    match shop.headsets {
        Headsets::Boat => println!("I got boat headsets"),
        Headsets::Jabra => println!("I got jabra headsets"),
        Headsets::Xbox => println!("I got xbox headsets"),
    }
}
