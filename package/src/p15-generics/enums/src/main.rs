fn main() {
    enum Options<T> {
        Some(T),
        None,
    }
    enum Result<T, E> {
        Ok(T),
        Err(E)
    }
}
