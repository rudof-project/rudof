use itertools::cloned;
use std::fmt::Debug;

/// Return an iterator that generates variants of the vector applying to each
/// element the function `F`
///
/// In case the value of `F` is None, the row will be skipped while if the value is `Some(x)`, then the corresponding element will be replaced by `x`.
///
/// ```
/// use rbe::deriv_n;
///
/// let vec = vec![1,2,3];
/// let sets = deriv_n(vec, |x: &i32| { Some(x + 100)}).collect::<Vec<_>>();
/// assert_eq!(sets, vec![
///   vec![101, 2, 3],
///   vec![1, 102, 3],
///   vec![1, 2, 103]
/// ])
///
/// ```
///
/// A similar example which doesn't generate iterator for the value 2
///
/// ```
/// use rbe::deriv_n;
///
/// let vec = vec![1,2,3];
/// let sets = deriv_n(vec, |x: &i32| {
///   match &x {
///     2 => None,
///     _ => Some(x + 100)
///   }
/// }).collect::<Vec<_>>();
/// assert_eq!(sets, vec![
///   vec![101, 2, 3],
///   vec![1, 2, 103]
/// ])
///
/// ```
pub fn deriv_n<T, F>(v: Vec<T>, d: F) -> DerivN<T, F> {
    DerivN {
        source: v,
        pos: 0,
        deriv: d,
    }
}

/// An adaptor of a `Vec<T>` that generates an iterator that contains variants of the vector
/// applying function `F` to each element.
///
/// The function `F` returns an `Option<T>` which means that if it returns `None`
/// that row is skipped while if it returns `Some(x)` then that value will be replaced by `x`.
///
///
pub struct DerivN<T, F> {
    source: Vec<T>,
    pos: usize,
    deriv: F,
}

impl<T, F: FnMut(&T) -> Option<T>> Iterator for DerivN<T, F>
where
    T: Clone + Debug,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.source.len() {
            let mut cloned: Vec<T> = cloned(self.source.iter()).collect();
            let current = &cloned[self.pos];
            match (self.deriv)(current) {
                None => {
                    // If it returns None we continue with the next position
                    self.pos += 1;
                    Self::next(self)
                }
                Some(d) => {
                    cloned[self.pos] = d;
                    self.pos += 1;
                    Some(cloned)
                }
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::deriv_n;

    #[test]
    fn example_deriv_n() {
        let vec = vec![1, 2, 3];

        let sets = deriv_n(vec, |x: &i32| Some(x + 100)).collect::<Vec<_>>();
        assert_eq!(
            sets,
            vec![vec![101, 2, 3], vec![1, 102, 3], vec![1, 2, 103]]
        )
    }

    #[test]
    fn test_deriv_n() {
        #[derive(Debug, Clone, PartialEq)]
        enum R {
            A(i32),
            B(i32),
            C(i32),
            D(Box<R>),
        }

        impl R {
            fn deriv(&self) -> Option<R> {
                match *self {
                    R::C(_) => None,
                    _ => Some(R::D(Box::new(self.clone()))),
                }
            }
        }

        let vs: Vec<R> = vec![R::A(1), R::B(2), R::C(4), R::A(3)];
        let mut results = deriv_n(vs, R::deriv);
        assert_eq!(
            vec![R::D(Box::new(R::A(1))), R::B(2), R::C(4), R::A(3)],
            results.next().unwrap()
        );
        assert_eq!(
            vec![R::A(1), R::D(Box::new(R::B(2))), R::C(4), R::A(3)],
            results.next().unwrap()
        );
        assert_eq!(
            vec![R::A(1), R::B(2), R::C(4), R::D(Box::new(R::A(3)))],
            results.next().unwrap()
        );
        assert_eq!(None, results.next());
    }
}
