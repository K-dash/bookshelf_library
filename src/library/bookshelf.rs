use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use super::book::Book;

pub struct Bookshelf {
    books: Vec<Book>,
    matcher: SkimMatcherV2,
}

impl Default for Bookshelf {
    fn default() -> Self {
        Self::new()
    }
}

impl Bookshelf {
    pub fn new() -> Self {
        let matcher = SkimMatcherV2::default();
        Self {
            books: Vec::new(),
            matcher,
        }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn search_books(&self, title_query: &str) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| self.matcher.fuzzy_match(&book.title, title_query).is_some())
            .collect()
    }

    pub fn remove_book(&mut self, book: &Book) -> Option<Book> {
        todo!("Implement `Bookshelf::remove_book`");
    }

    pub fn take_all_books(&mut self) -> Vec<Book> {
        todo!("Implement `Bookshelf::take_all_books`");
    }
}

#[cfg(test)]
mod tests {
    use super::{Book, Bookshelf};

    #[test]
    fn test_bookshelf() {
        let mut shelf = Bookshelf::new();
        let book1 = Book::new("すごいぞChatGPT！AIを使って学ぼうRust！", "山田太郎");

        let book2 = Book::new("Pythonプログラミング入門", "山田花子");
        shelf.add_book(book1);
        shelf.add_book(book2);

        let title_query = "chatgpt";
        let found_books = shelf.search_books(title_query);
        println!("Found books: {:?}", found_books);
    }
}
