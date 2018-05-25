trait Functor<A, B, MB> {
    fn fmap(&self, f: fn(&A) -> B) -> MB;
}

enum Maybe<T> {
    Nothing,
    Just(T),
}

impl<A, B> Functor<A, B, Maybe<B>> for Maybe<A> {
    fn fmap(&self, f: fn(&A) -> B) -> Maybe<B> {
        match self {
            Maybe::Just(x) => Maybe::Just(f(x)),
            Maybe::Nothing => Maybe::Nothing,
        }
    }
}

impl<T> Maybe<T> {
    fn get_just(&self) -> &T {
        match self {
            Maybe::Just(ref x) => x,
            Maybe::Nothing => panic!("error"),
        }
    }
}
