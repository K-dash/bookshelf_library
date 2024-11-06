pub mod library;

fn function_1() {
    let shelf = crate::library::bookshelf::Bookshelf::default();
}

fn function_2() {
    use library::bookshelf;
    let shelf = bookshelf::Bookshelf::default();
}
