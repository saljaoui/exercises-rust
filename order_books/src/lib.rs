pub mod library {
    pub mod writers {
        use crate::library::books::Book;
        pub struct Writer {
            pub first_name: String,
            pub last_name: String,
            pub books: Vec<Book>,
        }
    }
    pub mod books {
        #[derive(Debug)]
        pub struct Book {
            pub title: String,
            pub year: usize,
        }
    }
}

use crate::library::writers::Writer;

pub fn order_books(writer: &mut Writer) {
    println!("{:?}", writer.books);
}