use std::fmt::Display;

pub struct UsageCount {
    as_predicate: usize,
    as_subject: usize,
    as_object: usize,
    in_triple: usize,
}

impl UsageCount {
    pub fn new() -> Self {
        UsageCount {
            as_predicate: 0,
            as_subject: 0,
            as_object: 0,
            in_triple: 0,
        }
    }

    pub fn as_predicate(&self) -> usize {
        self.as_predicate
    }

    pub fn as_source(&self) -> usize {
        self.as_subject
    }

    pub fn as_object(&self) -> usize {
        self.as_object
    }

    pub fn in_triple(&self) -> usize {
        self.in_triple
    }

    pub fn increment_as_predicate(&mut self) {
        self.as_predicate += 1;
    }

    pub fn increment_as_subject(&mut self) {
        self.as_subject += 1;
    }

    pub fn increment_as_object(&mut self) {
        self.as_object += 1;
    }

    pub fn increment_in_triple(&mut self) {
        self.in_triple += 1;
    }
}

impl Display for UsageCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UsageCount {{ as_predicate: {}, as_subject: {}, as_object: {}, in_triple: {} }}",
            self.as_predicate, self.as_subject, self.as_object, self.in_triple
        )
    }
}
