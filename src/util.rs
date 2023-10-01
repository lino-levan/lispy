// Stolen from https://users.rust-lang.org/t/peekable-argument-there-has-to-be-a-simpler-way/6959/7
// TODO: understand this nonsense
pub trait PeekableIterator: Iterator {
    fn peek(&mut self) -> Option<&Self::Item>;
}

impl<I: std::iter::Iterator> PeekableIterator for std::iter::Peekable<I> {
    fn peek(&mut self) -> Option<&Self::Item> {
        std::iter::Peekable::peek(self)
    }
}
