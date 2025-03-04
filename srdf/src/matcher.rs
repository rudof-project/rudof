pub trait Matcher<T> {
    fn value(&self) -> Option<T>;
    fn matches<M: Into<T> + Clone>(&self, other: &M) -> bool;
}

pub struct Any;

impl<T> Matcher<T> for Any {
    fn value(&self) -> Option<T> {
        None
    }

    fn matches<M: Into<T> + Clone>(&self, _other: &M) -> bool {
        true
    }
}

impl<T, I> Matcher<T> for I
where
    T: PartialEq,
    I: Into<T> + Clone,
{
    fn value(&self) -> Option<T> {
        Some(self.clone().into())
    }

    fn matches<M: Into<T> + Clone>(&self, other: &M) -> bool {
        self.clone().into() == other.clone().into()
    }
}
