#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub borrowed_books: Vec<u32>,
}

pub struct UserManger {
    users: Vec<User>,
}

impl UserManger {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }

    pub fn register_user(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn get_user(&self, user_id: u32) -> Option<&User> {
        self.users.iter().find(|&u| u.id == user_id)
    }

    pub fn borrow_book(&mut self, user_id: u32, book_id: u32) {
        if let Some(user) = self.users.iter_mut().find(|u| u.id == user_id) {
            user.borrowed_books.push(book_id);
        }
    }
}
