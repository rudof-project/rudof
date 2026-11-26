use crate::PrefixMap;

pub trait Show {
    fn show(&self, pm: &PrefixMap) -> String;
}

impl<S: Show> Show for Option<&S> {
    fn show(&self, pm: &PrefixMap) -> String {
        self.map(|i| i.show(pm)).unwrap_or_default()
    }
}
