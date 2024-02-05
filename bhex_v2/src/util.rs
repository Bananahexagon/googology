pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn left(self) -> Option<L> {
        match self {
            Self::Left(l) => Some(l),
            Self::Right(r) => None,
        }
    }
    pub fn right(self) -> Option<R> {
        match self {
            Self::Left(l) => None,
            Self::Right(r) => Some(r),
        }
    }
}
