use crate::rdf_core::vocabs::RdfVocabulary;
use crate::vocab_term;

/// EARL 1.0 vocabulary terms
/// See the [EARL 1.0 Schema](https://www.w3.org/TR/EARL10-Schema/).
pub struct EarlVocab;

impl RdfVocabulary for EarlVocab {
    const BASE: &'static str = "http://www.w3.org/ns/earl#";
}

// Base
vocab_term!(EarlVocab, EARL, "");

// Classes
vocab_term!(EarlVocab, EARL_ASSERTION, "Assertion");
vocab_term!(EarlVocab, EARL_ASSERTOR, "Assertor");
vocab_term!(EarlVocab, EARL_TEST_SUBJECT, "TestSubject");
vocab_term!(EarlVocab, EARL_TEST_CRITERION, "TestCriterion");
vocab_term!(EarlVocab, EARL_TEST_CASE, "TestCase");
vocab_term!(EarlVocab, EARL_TEST_REQUIREMENT, "TestRequirement");
vocab_term!(EarlVocab, EARL_TEST_RESULT, "TestResult");
vocab_term!(EarlVocab, EARL_TEST_MODE, "TestMode");
vocab_term!(EarlVocab, EARL_OUTCOME_VALUE, "OutcomeValue");
vocab_term!(EarlVocab, EARL_SOFTWARE, "Software");

// Outcome classes
vocab_term!(EarlVocab, EARL_PASS, "Pass");
vocab_term!(EarlVocab, EARL_FAIL, "Fail");
vocab_term!(EarlVocab, EARL_CANNOT_TELL, "CannotTell");
vocab_term!(EarlVocab, EARL_NOT_APPLICABLE, "NotApplicable");
vocab_term!(EarlVocab, EARL_NOT_TESTED, "NotTested");

// Properties
vocab_term!(EarlVocab, EARL_ASSERTED_BY, "assertedBy");
vocab_term!(EarlVocab, EARL_SUBJECT, "subject");
vocab_term!(EarlVocab, EARL_TEST, "test");
vocab_term!(EarlVocab, EARL_RESULT, "result");
vocab_term!(EarlVocab, EARL_MODE, "mode");
vocab_term!(EarlVocab, EARL_OUTCOME, "outcome");
vocab_term!(EarlVocab, EARL_POINTER, "pointer");
vocab_term!(EarlVocab, EARL_INFO, "info");
vocab_term!(EarlVocab, EARL_MAIN_ASSERTOR, "mainAssertor");

// Outcome instances
vocab_term!(EarlVocab, EARL_PASSED, "passed");
vocab_term!(EarlVocab, EARL_FAILED, "failed");
vocab_term!(EarlVocab, EARL_CANT_TELL, "cantTell");
vocab_term!(EarlVocab, EARL_INAPPLICABLE, "inapplicable");
vocab_term!(EarlVocab, EARL_UNTESTED, "untested");

// Test mode instances
vocab_term!(EarlVocab, EARL_AUTOMATIC, "automatic");
vocab_term!(EarlVocab, EARL_MANUAL, "manual");
vocab_term!(EarlVocab, EARL_SEMI_AUTO, "semiAuto");
vocab_term!(EarlVocab, EARL_UNDISCLOSED, "undisclosed");
vocab_term!(EarlVocab, EARL_UNKNOWN_MODE, "unknownMode");
