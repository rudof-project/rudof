//! Generic `(node, shape label) -> outcome` memoization cache with an
//! observer hook, shared across rudof's validators.
//!
//! A validator proves `(node, shape label)` pairs, possibly several times
//! via different recursive paths. [`Typing`] is the cache that lets a pair
//! be proved once and reused; [`TypingObserver`] lets a caller be notified
//! of each newly-cached result as it happens, e.g. to show progress or to
//! recover partial results if validation is interrupted.
//!
//! `N` is the node type, `L` the shape-label type, `E` the error type
//! explaining why a pair doesn't conform, and `Ev` the evidence type
//! explaining why it does (e.g. for ShEx: `Node`, `ShapeLabelIdx`,
//! `ValidatorError`, `Reason`).

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;

use either::Either;

/// The outcome of proving one `(node, shape label)` pair: either it
/// conforms (with a list of `Ev` evidence of why), or it doesn't (with a
/// list of `E` errors explaining why not).
pub type ValidationResult<E, Ev> = Either<Vec<E>, Vec<Ev>>;

/// A cache of `(N, L) -> ValidationResult<E, Ev>` built up during
/// validation. Results are memoized here and reused across recursive
/// calls, so a pair reachable from several branches is proved once.
pub trait Typing<N, L, E, Ev> {
    fn get(&self, key: &(N, L)) -> Option<&ValidationResult<E, Ev>>;
    fn insert(&mut self, key: (N, L), value: ValidationResult<E, Ev>);
}

/// Notified whenever an [`ObservableTyping`] caches a newly-proved
/// `(node, shape)` result. Lets a caller surface validation progress as it
/// happens, or collect partial results if validation is later interrupted.
pub trait TypingObserver<N, L, E, Ev>: Send + Sync + Debug {
    fn on_insert(&self, key: &(N, L), value: &ValidationResult<E, Ev>);
}

/// Default [`Typing`] implementation: a plain memoization cache that also
/// notifies an optional [`TypingObserver`] on every insert.
#[derive(Debug, Clone)]
pub struct ObservableTyping<N, L, E, Ev>
where
    N: Eq + Hash + Clone + Debug,
    L: Eq + Hash + Clone + Debug,
{
    map: HashMap<(N, L), ValidationResult<E, Ev>>,
    observer: Option<Arc<dyn TypingObserver<N, L, E, Ev>>>,
}

impl<N, L, E, Ev> ObservableTyping<N, L, E, Ev>
where
    N: Eq + Hash + Clone + Debug,
    L: Eq + Hash + Clone + Debug,
{
    pub fn new(observer: Option<Arc<dyn TypingObserver<N, L, E, Ev>>>) -> Self {
        ObservableTyping {
            map: HashMap::new(),
            observer,
        }
    }
}

// Not `#[derive(Default)]`: that would force `N`/`L`/`E`/`Ev`: Default too,
// which none of this crate's actual instantiations need or provide.
impl<N, L, E, Ev> Default for ObservableTyping<N, L, E, Ev>
where
    N: Eq + Hash + Clone + Debug,
    L: Eq + Hash + Clone + Debug,
{
    fn default() -> Self {
        Self::new(None)
    }
}

impl<N, L, E, Ev> Typing<N, L, E, Ev> for ObservableTyping<N, L, E, Ev>
where
    N: Eq + Hash + Clone + Debug,
    L: Eq + Hash + Clone + Debug,
{
    fn get(&self, key: &(N, L)) -> Option<&ValidationResult<E, Ev>> {
        self.map.get(key)
    }

    fn insert(&mut self, key: (N, L), value: ValidationResult<E, Ev>) {
        if let Some(observer) = &self.observer {
            observer.on_insert(&key, &value);
        }
        self.map.insert(key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[test]
    fn insert_then_get_returns_the_cached_value() {
        let mut typing: ObservableTyping<i32, i32, String, String> = ObservableTyping::default();
        assert!(typing.get(&(1, 2)).is_none());
        typing.insert((1, 2), Either::Right(vec!["ok".to_string()]));
        assert_eq!(typing.get(&(1, 2)), Some(&Either::Right(vec!["ok".to_string()])));
    }

    #[test]
    fn get_on_a_different_key_is_none() {
        let mut typing: ObservableTyping<i32, i32, String, String> = ObservableTyping::default();
        typing.insert((1, 2), Either::Left(vec!["bad".to_string()]));
        assert!(typing.get(&(1, 3)).is_none());
        assert!(typing.get(&(2, 2)).is_none());
    }

    #[derive(Debug, Default)]
    struct RecordingObserver {
        seen: Mutex<Vec<(i32, i32, ValidationResult<String, String>)>>,
    }

    impl TypingObserver<i32, i32, String, String> for RecordingObserver {
        fn on_insert(&self, key: &(i32, i32), value: &ValidationResult<String, String>) {
            self.seen.lock().unwrap().push((key.0, key.1, value.clone()));
        }
    }

    #[test]
    fn observer_is_notified_on_insert_with_the_right_key_and_value() {
        let observer = Arc::new(RecordingObserver::default());
        let mut typing: ObservableTyping<i32, i32, String, String> = ObservableTyping::new(Some(observer.clone()));
        typing.insert((7, 9), Either::Right(vec!["evidence".to_string()]));
        let seen = observer.seen.lock().unwrap();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0], (7, 9, Either::Right(vec!["evidence".to_string()])));
    }

    #[test]
    fn no_observer_means_no_panic_and_no_notification() {
        let mut typing: ObservableTyping<i32, i32, String, String> = ObservableTyping::default();
        typing.insert((1, 1), Either::Right(vec![]));
        assert!(typing.get(&(1, 1)).is_some());
    }
}
