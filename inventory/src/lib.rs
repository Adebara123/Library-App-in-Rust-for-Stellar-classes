

#[derive(Debug, Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub genre: Genre,
    pub is_available: bool,
}

#[derive(Debug, Clone)]
pub enum Genre {
    Fiction,
    NonFiction,
    Science,
    History,
    Fantasy,
    Other(String)
}

pub struct Inventory {
    books: Vec<Book>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            books: Vec::new()
        }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book)
    }

    pub fn remove_book (&mut self, book_id: u32) -> Option<Book> {
        if let Some(pos) = self.books.iter().position(|b| b.id == book_id) {
            Some(self.books.remove(pos))
        } else {
            None
        }
    }

    // [book1, book2, book3, book4, book5]

    pub fn get_book(&self, book_id: u32) -> Option<&Book> {
        self.books.iter().find(|&b| b.id == book_id)
    }
}
