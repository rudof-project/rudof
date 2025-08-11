// Specialized field generators for specific domains or use cases

pub mod person;
pub mod organization;
pub mod academic;
pub mod temporal;

pub use person::PersonGenerator;
pub use organization::OrganizationGenerator;
pub use academic::AcademicGenerator;
pub use temporal::TemporalGenerator;
