use inventory::{Book, Inventory};
use lib_users::UserManger;

pub struct BorrowingService;

impl  BorrowingService {
    
    pub fn new() -> Self {
        BorrowingService
    }

    pub fn borrow_book (&self, inventory: &mut Inventory, user_manager: &mut UserManger, user_id: u32, book_id: u32) -> Result<(), String> {

        // check if the book exist and is available 

        let book = inventory.get_book(book_id).ok_or_else(|| "Book not found".to_string())?;

        if !book.is_available {
            return Err(String::from("Book not Available"));
        }

        // check if th user exists or hasn't borrowed more than 3 books 

        // update the book availability 

        // Add book to user's borrowed books 

        todo!()
    }
}