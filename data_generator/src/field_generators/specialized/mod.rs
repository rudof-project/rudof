// Specialized field generators for specific domains or use cases

pub mod academic;
pub mod organization;
pub mod person;
pub mod temporal;

pub use academic::AcademicGenerator;
pub use organization::OrganizationGenerator;
pub use person::PersonGenerator;
pub use temporal::TemporalGenerator;
