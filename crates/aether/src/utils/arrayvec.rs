use arrayvec::{Array, ArrayVec};

pub trait ArrayVecExt<A: Array> {
    fn of<AOf>(array: AOf) -> Self
    where
        AOf: Array<Item = A::Item>;
}

impl<A: Array> ArrayVecExt<A> for ArrayVec<A> {
    /// Initialize an `ArrayVec` from an array of shorter length
    ///
    /// See <https://github.com/bluss/arrayvec/issues/128#issuecomment-536330697>
    fn of<AOf>(array: AOf) -> Self
    where
        AOf: Array<Item = A::Item>,
    {
        let mut result = Self::new();
        let array = ArrayVec::from(array);
        for x in array {
            result.push(x);
        }
        result
    }
}
