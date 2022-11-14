//Topic: Getting used to with Struct

//
//Requirements
//
/*
1- Define a struct Book having two fields author:Author and price:i32,
2- Define a struct Author having two fields one is name and stars,
3- you gonna purchase a book only if author has rating >= 4.9 stars .

*/

impl Clone for Author {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            stars: self.stars,
        }
    }
}

struct Author {
    name: String,
    stars: f32,
}
struct Book {
    author: Author,
    price: i32,
}
fn main() {
    let authors = vec![
        Author {
            name: "Alex".to_string(),
            stars: 4.5,
        },
        Author {
            name: "Brittney".to_string(),
            stars: 5.0,
        },
        Author {
            name: "Camilla".to_string(),
            stars: 4.9,
        },
    ];
    let books = vec![
        Book {
            author: authors[0].clone(),
            price: 500,
        },
        Book {
            author: authors[1].clone(),
            price: 800,
        },
        Book {
            author: authors[1].clone(),
            price: 900,
        },
        Book {
            author: authors[2].clone(),
            price: 400,
        },
    ];
    let mut bill = 0;
    for book in books.iter() {
        if book.author.stars >= 4.9 {
            bill += book.price;
        }
    }
    println!("Total price to pay: {}", bill);
}
