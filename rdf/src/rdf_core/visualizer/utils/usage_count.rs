use std::fmt::Display;

/// Tracks how often an entity is used in different roles within triples.
pub struct UsageCount {
    /// Number of times the entity is used as a predicate.
    as_predicate: usize,
    /// Number of times the entity is used as a predicate in a triple.
    as_predicate_in_triple: usize,
    /// Number of times the entity is used as a subject.
    as_subject: usize,
    /// Number of times the entity is used as a subject in a triple.
    as_subject_in_triple: usize,
    /// Number of times the entity is used as an object.
    as_object: usize,
    /// Number of times the entity is used as an object in a triple.
    as_object_in_triple: usize,
}

impl UsageCount {
    /// Creates a new [`UsageCount`] with all counters initialized to zero.
    pub fn new() -> Self {
        UsageCount {
            as_predicate: 0,
            as_subject: 0,
            as_object: 0,
            as_predicate_in_triple: 0,
            as_subject_in_triple: 0,
            as_object_in_triple: 0,
        }
    }

     /// Returns the number of times the entity was used as a predicate.
    pub fn as_predicate(&self) -> usize {
        self.as_predicate
    }

     /// Returns the number of times the entity was used as a subject.
    pub fn as_subject(&self) -> usize {
        self.as_subject
    }

     /// Returns the number of times the entity was used as an object.
    pub fn as_object(&self) -> usize {
        self.as_object
    }

    /// Returns `true` if the entity appears in at least one triple.
    pub fn in_triple(&self) -> bool {
        self.as_predicate_in_triple > 0
            || self.as_subject_in_triple > 0
            || self.as_object_in_triple > 0
    }

    /// Increments the predicate usage counter.
    pub fn increment_as_predicate(&mut self) {
        self.as_predicate += 1;
    }

    /// Increments the subject usage counter.
    pub fn increment_as_subject(&mut self) {
        self.as_subject += 1;
    }

    /// Increments the object usage counter.
    pub fn increment_as_object(&mut self) {
        self.as_object += 1;
    }

    /// Increments the predicate-in-triple counter.
    pub fn increment_as_predicate_in_triple(&mut self) {
        self.as_predicate_in_triple += 1;
    }

    /// Increments the subject-in-triple counter.
    pub fn increment_as_subject_in_triple(&mut self) {
        self.as_subject_in_triple += 1;
    }

    /// Increments the object-in-triple counter.
    pub fn increment_as_object_in_triple(&mut self) {
        self.as_object_in_triple += 1;
    }
}

impl Display for UsageCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UsageCount {{ as_predicate: {}, as_subject: {}, as_object: {}, as_predicate_in_triple: {}, as_subject_in_triple: {}, as_object_in_triple: {} }}",
            self.as_predicate,
            self.as_subject,
            self.as_object,
            self.as_predicate_in_triple,
            self.as_subject_in_triple,
            self.as_object_in_triple
        )
    }
}

impl Default for UsageCount {
    fn default() -> Self {
        UsageCount::new()
    }
}
