mod data_manager;

pub(crate) trait Store<S> {
    fn store(&self) -> &S;
}