//! Interpolation of variable names in result messages.
//!
//! SHACL lets a message literal contain `{?varName}` and `{$varName}` blocks.
//! The validator must replace each block with a string that shows the value of that variable.
//!
//! - SHACL 1.0: <https://www.w3.org/TR/shacl/#results-message>
//! - SHACL 1.2: <https://www.w3.org/TR/shacl12-sparql/#sparql-constraints-variables>
//!
//! The functions in this module do not know where the values come from.
//! The caller gives a resolver function that returns the value of a variable name.
//! Thus, you can use these functions for each type of result message.

use crate::types::MessageMap;
use rudof_rdf::rdf_core::term::Object;

/// Replaces the `{?varName}` and `{$varName}` blocks in `template`.
///
/// The function calls `resolve` with the variable name, without the `?` or `$` prefix.
///
/// - If `resolve` gives a value, the function puts the value in place of the block.
/// - If `resolve` gives no value, the function keeps the block unchanged.
/// - The function keeps all other text unchanged, also braces that are not a variable block.
///
/// A variable name must have one or more characters.
/// Each character must be a letter, a digit or an underscore (`_`).
pub fn interpolate_message<F>(template: &str, resolve: F) -> String
where
    F: Fn(&str) -> Option<String>,
{
    let mut result = String::with_capacity(template.len());
    let mut rest = template;

    while let Some(start) = rest.find('{') {
        result.push_str(&rest[..start]);
        let block = &rest[start..];

        match parse_variable_block(block) {
            Some((name, block_len)) => {
                match resolve(name) {
                    Some(value) => result.push_str(&value),
                    None => result.push_str(&block[..block_len]),
                }
                rest = &block[block_len..];
            },
            None => {
                // This brace does not start a variable block. Keep it and continue after it.
                result.push('{');
                rest = &block[1..];
            },
        }
    }

    result.push_str(rest);
    result
}

/// Reads a variable block at the start of `block`.
///
/// The text must start with `{?name}` or `{$name}`.
/// If it does, the function gives the variable name and the length of the block in bytes.
/// If it does not, the function gives `None`.
fn parse_variable_block(block: &str) -> Option<(&str, usize)> {
    let after_brace = block.strip_prefix('{')?;
    let after_prefix = after_brace
        .strip_prefix('?')
        .or_else(|| after_brace.strip_prefix('$'))?;
    let end = after_prefix.find('}')?;
    let name = &after_prefix[..end];

    if name.is_empty() || !name.chars().all(is_variable_char) {
        return None;
    }

    // The block has the opening brace, the prefix, the name and the closing brace.
    Some((name, 2 + name.len() + 1))
}

fn is_variable_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Gives the string that shows `object` in a result message.
///
/// - For a literal, the function gives the lexical form, without quotes, language tag or datatype.
/// - For an IRI, the function gives the full IRI, without angle brackets.
/// - For a blank node, the function gives the blank node label with the `_:` prefix.
pub fn object_to_message_value(object: &Object) -> String {
    match object {
        Object::Literal(literal) => literal.lexical_form(),
        other => other.to_string(),
    }
}

impl MessageMap {
    /// Replaces the variable blocks in each message of this map.
    ///
    /// The language tag of each message does not change.
    /// Refer to [`interpolate_message`] for the replacement rules.
    pub fn interpolate<F>(&self, resolve: F) -> MessageMap
    where
        F: Fn(&str) -> Option<String>,
    {
        self.iter().fold(MessageMap::new(), |map, (lang, message)| {
            map.with_message(lang.clone(), interpolate_message(message, &resolve))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rudof_iri::IriS;
    use rudof_rdf::rdf_core::term::literal::{ConcreteLiteral, Lang};
    use std::collections::HashMap;

    fn resolver(bindings: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let map: HashMap<String, String> = bindings.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        move |name| map.get(name).cloned()
    }

    #[test]
    fn keeps_text_without_blocks() {
        let result = interpolate_message("No variables here", resolver(&[("x", "1")]));
        assert_eq!(result, "No variables here");
    }

    #[test]
    fn replaces_question_mark_block() {
        let result = interpolate_message("Value is {?value}", resolver(&[("value", "42")]));
        assert_eq!(result, "Value is 42");
    }

    #[test]
    fn replaces_dollar_block() {
        let result = interpolate_message("Focus node is {$this}", resolver(&[("this", "http://example.org/a")]));
        assert_eq!(result, "Focus node is http://example.org/a");
    }

    #[test]
    fn replaces_many_blocks_and_repeated_blocks() {
        let result = interpolate_message(
            "{?a} and {$b}, again {?a}.",
            resolver(&[("a", "first"), ("b", "second")]),
        );
        assert_eq!(result, "first and second, again first.");
    }

    #[test]
    fn replaces_blocks_next_to_each_other() {
        let result = interpolate_message("{?a}{$b}", resolver(&[("a", "x"), ("b", "y")]));
        assert_eq!(result, "xy");
    }

    #[test]
    fn keeps_block_when_variable_has_no_value() {
        let result = interpolate_message("Value is {?unknown}", resolver(&[]));
        assert_eq!(result, "Value is {?unknown}");
    }

    #[test]
    fn keeps_braces_that_are_not_variable_blocks() {
        let template = "Set {a, b}, empty {?}, {x}, {?bad name}, {$} and { ?a }";
        let result = interpolate_message(template, resolver(&[("a", "X"), ("x", "X")]));
        assert_eq!(result, template);
    }

    #[test]
    fn keeps_block_without_closing_brace() {
        let result = interpolate_message("Open {?value and more", resolver(&[("value", "1")]));
        assert_eq!(result, "Open {?value and more");
    }

    #[test]
    fn accepts_digits_underscore_and_non_ascii_letters_in_names() {
        let result = interpolate_message("{?var_1} {$número}", resolver(&[("var_1", "one"), ("número", "two")]));
        assert_eq!(result, "one two");
    }

    #[test]
    fn does_not_interpolate_again_the_inserted_values() {
        let result = interpolate_message("{?a}", resolver(&[("a", "{?b}"), ("b", "wrong")]));
        assert_eq!(result, "{?b}");
    }

    #[test]
    fn keeps_non_ascii_text() {
        let result = interpolate_message("Wert für {?v} ist ungültig ✗", resolver(&[("v", "ä")]));
        assert_eq!(result, "Wert für ä ist ungültig ✗");
    }

    #[test]
    fn literal_value_is_the_lexical_form() {
        let lit = Object::Literal(ConcreteLiteral::StringLiteral {
            lexical_form: "hello".to_string(),
            lang: Some(Lang::new("en").unwrap()),
        });
        assert_eq!(object_to_message_value(&lit), "hello");

        let typed = Object::Literal(ConcreteLiteral::DatatypeLiteral {
            lexical_form: "2020-01-01".to_string(),
            datatype: IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#date").into(),
        });
        assert_eq!(object_to_message_value(&typed), "2020-01-01");
    }

    #[test]
    fn iri_value_is_the_full_iri() {
        let iri = Object::iri(IriS::new_unchecked("http://example.org/alice"));
        assert_eq!(object_to_message_value(&iri), "http://example.org/alice");
    }

    #[test]
    fn blank_node_value_has_prefix() {
        let bnode = Object::bnode("b0".to_string());
        assert_eq!(object_to_message_value(&bnode), "_:b0");
    }

    #[test]
    fn message_map_interpolates_each_language() {
        let en = Lang::new("en").unwrap();
        let es = Lang::new("es").unwrap();
        let messages = MessageMap::new()
            .with_message(None, "Bad value {?v}".to_string())
            .with_message(Some(en.clone()), "Value {?v} is bad".to_string())
            .with_message(Some(es.clone()), "El valor {$v} es incorrecto".to_string());

        let result = messages.interpolate(resolver(&[("v", "7")]));

        assert_eq!(result.get(None), Some(&"Bad value 7".to_string()));
        assert_eq!(result.get(Some(&en)), Some(&"Value 7 is bad".to_string()));
        assert_eq!(result.get(Some(&es)), Some(&"El valor 7 es incorrecto".to_string()));
    }
}
