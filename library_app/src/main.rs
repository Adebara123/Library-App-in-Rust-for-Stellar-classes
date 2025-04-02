use lib_boworring::BorrowingService;
use inventory::{Book, Genre, Inventory};
use lib_users::{User, UserManger};

use std::io::{self, Write};

fn main () {

    let mut inventory = Inventory::new();
    let mut user_manager = UserManger::new();
    let borrowing_service = BorrowingService::new();

    loop {
        println!("\nLibrary Management System");
        println!("1. Add Book");
        println!("2. Register User");
        println!("3. Borrow Book");
        println!("4. Return Book");
        println!("5. List Books");
        println!("6. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<u32>().unwrap_or(0);

        match choice {
            1 => {
                println!("Enter the Book details: ");
                print!("ID: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id: u32 = id.trim().parse().unwrap_or(0);

                print!("Title: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();

                print!("Author: ");
                io::stdout().flush().unwrap();
                let mut author = String::new();
                io::stdin().read_line(&mut author).unwrap();

                let book = Book {
                    id,
                    title: title.trim().to_string(),
                    author: author.trim().to_string(),
                    genre: Genre::Other("Unknown".to_string()),
                    is_available: true,
                };

                inventory.add_book(book);

                println!("Book added.");

            }

            2 => {
                println!("Enter user details:");
                print!("ID: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id: u32 = id.trim().parse().unwrap_or(0);

                print!("Name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();

                let user = User {
                    id,
                    name: name.trim().to_string(),
                    borrowed_books: Vec::new(),
                };

                user_manager.register_user(user);
                println!("User registered");
            }

            3 => {
                print!("Enter user ID: ");
                io::stdout().flush().unwrap();
                let mut user_id = String::new();
                io::stdin().read_line(&mut user_id).unwrap();
                let user_id: u32 = user_id.trim().parse().unwrap_or(0);


                print!("Enter Book ID: ");
                io::stdout().flush().unwrap();
                let mut book_id = String::new();
                io::stdin().read_line(&mut book_id).unwrap();
                let book_id: u32 = book_id.trim().parse().unwrap_or(0);

                match borrowing_service.borrow_book(&mut inventory, &mut user_manager, user_id, book_id)  {
                    Ok(_) => println!("Book borrowed sucessfully."),
                    Err(err) => println!("Error: {}", err),
                }
            }

            4 => {
                print!("Enter user ID: ");
                io::stdout().flush().unwrap();
                let mut user_id = String::new();
                io::stdin().read_line(&mut user_id).unwrap();
                let user_id: u32 = user_id.trim().parse().unwrap_or(0);


                print!("Enter Book ID: ");
                io::stdout().flush().unwrap();
                let mut book_id = String::new();
                io::stdin().read_line(&mut book_id).unwrap();
                let book_id: u32 = book_id.trim().parse().unwrap_or(0);

                match borrowing_service.return_book(&mut inventory, &mut user_manager, user_id, book_id) {
                    Ok(_) => println!("Book returned successfully."),
                    Err(err) => println!("Error: {}", err),
                }

            }

            5 => {
                println!("Listsing all books:");
                for book in inventory.list_books() {
                    println!(
                        "ID: {}, Title: {}, Author: {}, Available: {}", 
                        book.id, book.title, book.author, book.is_available
                    );
                }
            } 

            6 => {
                println!("Exiting ...");
                break;
            }

            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }

}


