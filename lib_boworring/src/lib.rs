use inventory::{Book, Inventory};
use lib_users::{User, UserManger};

pub struct BorrowingService;

impl BorrowingService {
    pub fn new() -> Self {
        BorrowingService
    }

    pub fn borrow_book(
        &self,
        inventory: &mut Inventory,
        user_manager: &mut UserManger,
        user_id: u32,
        book_id: u32,
    ) -> Result<(), String> {
        // check if the book exist and is available

        let book = inventory
            .get_book(book_id)
            .ok_or_else(|| "Book not found".to_string())?;

        if !book.is_available {
            return Err(String::from("Book not Available"));
        }

        // check if the user exists or hasn't borrowed more than 3 books

        let user = user_manager
            .get_user(user_id)
            .ok_or_else(|| "User not found .".to_string())?;

        if user.borrowed_books.len() >= 3 {
            return Err(String::from(
                "User cannot borrow more tham 3 books at a time .",
            ));
        }

        // update the book availability
        inventory.update_book_availability(book_id, false)?;

        // Add book to user's borrowed books
        user_manager.borrow_book(user_id, book_id);

        Ok(())
    }

    pub fn return_book(
        &self,
        inventory: &mut Inventory,
        user_manager: &mut UserManger,
        user_id: u32,
        book_id: u32,
    ) -> Result<(), String> {
        // First try to return the book in the user manager 

        user_manager.return_book(user_id, book_id);

        // If successful, update the book's availablity in the inventory
        inventory.update_book_availability(book_id, true)?;

        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use lib_users::User;
    use inventory::{Genre, Book};

    fn setup() -> (Inventory, UserManger, BorrowingService) {
        let mut inventory = Inventory::new();
        let mut user_manager = UserManger::new();
        let borrowing_service = BorrowingService::new();

        // Add a book 

        let book = Book {
            id: 1,
            title: String::from("Test Book"),
            author: String::from("Test Author"),
            genre: Genre::Fiction,
            is_available: true,
        };

        inventory.add_book(book);

        // Add a user 

        let user = User {
            id: 1,
            name: String::from("Test User"),
            borrowed_books: Vec::new(),
        };

        user_manager.register_user(user);

       (inventory, user_manager, borrowing_service)
    }


    // Test borrow book and return book 
    // Borrow unavailable book 

    #[test]
    fn test_successful_borrow_and_return_book () {
        let (mut inventory, mut user_manager, borrowing_service) = setup();

        // Borrow the book 
        assert!(borrowing_service.borrow_book(&mut inventory, &mut user_manager, 1, 1).is_ok());
        assert!(!inventory.get_book(1).unwrap().is_available);
        assert!(user_manager.get_user(1).unwrap().borrowed_books.contains(&1));

        // Return the book 
        assert!(borrowing_service.return_book(&mut inventory, &mut user_manager, 1, 1).is_ok());
        assert!(inventory.get_book(1).unwrap().is_available);
        assert!(!user_manager.get_user(1).unwrap().borrowed_books.contains(&1));
    }

}