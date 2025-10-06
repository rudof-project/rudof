use crate::field_generators::{FieldGenerator, GenerationContext};
use crate::{Result, DataGeneratorError};
use rand::Rng;

/// Pattern-based field generator that generates strings matching regex patterns
pub struct PatternGenerator;

impl FieldGenerator for PatternGenerator {
    fn generate(&self, context: &GenerationContext) -> Result<String> {
        // Check if a pattern parameter is provided
        if let Some(pattern_value) = context.parameters.get("pattern") {
            if let Some(pattern) = pattern_value.as_str() {
                return self.generate_from_pattern(pattern);
            }
        }

        // Fallback to heuristic-based generation based on property name
        self.generate_heuristic(context)
    }

    fn name(&self) -> &str {
        "pattern"
    }

    fn supported_datatypes(&self) -> Vec<String> {
        vec!["http://www.w3.org/2001/XMLSchema#string".to_string()]
    }
}

impl PatternGenerator {
    /// Generate a string that matches the given regex pattern
    fn generate_from_pattern(&self, pattern: &str) -> Result<String> {
        // For now, implement common pattern matching
        // In a full implementation, you'd use a regex-to-string generation library
        
        let mut rng = rand::thread_rng();
        
        // Handle common patterns - check international first
        if pattern.contains("\\+1-\\d{3}-\\d{3}-\\d{4}") || pattern.contains("\\+1\\-\\d{3}\\-\\d{3}\\-\\d{4}") {
            // International phone number with country code
            return Ok(format!("+1-{:03}-{:03}-{:04}", 
                rng.gen_range(100..999),
                rng.gen_range(100..999), 
                rng.gen_range(1000..9999)
            ));
        }
        
        if pattern.contains("\\d{3}-\\d{3}-\\d{4}") || pattern.contains("\\d{3}\\-\\d{3}\\-\\d{4}") {
            // Regular phone number pattern
            return Ok(format!("{:03}-{:03}-{:04}", 
                rng.gen_range(100..999),
                rng.gen_range(100..999), 
                rng.gen_range(1000..9999)
            ));
        }
        
        if pattern.contains("\\+1-\\d{3}-\\d{3}-\\d{4}") {
            // US phone number with country code
            return Ok(format!("+1-{:03}-{:03}-{:04}", 
                rng.gen_range(100..999),
                rng.gen_range(100..999), 
                rng.gen_range(1000..9999)
            ));
        }
        
        if pattern.contains("@") || pattern.contains("[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}") {
            // Email pattern
            let domains = ["example.com", "test.org", "sample.edu", "demo.net"];
            let usernames = ["user", "admin", "test", "demo", "john.doe", "jane.smith"];
            return Ok(format!("{}{}@{}", 
                usernames[rng.gen_range(0..usernames.len())],
                rng.gen_range(1..100),
                domains[rng.gen_range(0..domains.len())]
            ));
        }
        
        if pattern.contains("[A-Z]{2,3}\\d{4,6}") {
            // ID pattern like "AB1234" or "XYZ123456"
            let letters: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
            let letter_count = if pattern.contains("{2,3}") { rng.gen_range(2..=3) } else { 2 };
            let digit_count = if pattern.contains("{4,6}") { rng.gen_range(4..=6) } else { 4 };
            
            let mut result = String::new();
            for _ in 0..letter_count {
                result.push(letters[rng.gen_range(0..letters.len())]);
            }
            for _ in 0..digit_count {
                result.push_str(&rng.gen_range(0..10).to_string());
            }
            return Ok(result);
        }
        
        if pattern.contains("\\d{4}-\\d{2}-\\d{2}") {
            // Date pattern YYYY-MM-DD
            let year = rng.gen_range(1980..=2024);
            let month = rng.gen_range(1..=12);
            let day = rng.gen_range(1..=28); // Safe day range
            return Ok(format!("{year:04}-{month:02}-{day:02}"));
        }
        
        if pattern.contains("\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}") {
            // IP address pattern
            return Ok(format!("{}.{}.{}.{}", 
                rng.gen_range(1..255),
                rng.gen_range(0..255),
                rng.gen_range(0..255), 
                rng.gen_range(1..255)
            ));
        }
        
        if pattern.contains("https?://") {
            // URL pattern
            let protocols = ["http", "https"];
            let domains = ["example.com", "test.org", "sample.net"];
            let paths = ["", "/page", "/api/v1", "/data", "/users"];
            return Ok(format!("{}://{}{}", 
                protocols[rng.gen_range(0..protocols.len())],
                domains[rng.gen_range(0..domains.len())],
                paths[rng.gen_range(0..paths.len())]
            ));
        }
        
        if pattern.contains("#[0-9A-Fa-f]{6}") || pattern.contains("#[0-9A-F]{6}") {
            // Hex color pattern like #FF0000
            return Ok(format!("#{:06X}", rng.gen_range(0..0x1000000)));
        }
        
        if pattern.contains("[A-Z]{3}\\d{3}") {
            // License plate pattern like ABC123
            let letters: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
            let mut result = String::new();
            for _ in 0..3 {
                result.push(letters[rng.gen_range(0..letters.len())]);
            }
            for _ in 0..3 {
                result.push_str(&rng.gen_range(0..10).to_string());
            }
            return Ok(result);
        }
        
        if pattern.contains("\\d{3}-\\d{2}-\\d{4}") {
            // SSN pattern like 123-45-6789
            return Ok(format!("{:03}-{:02}-{:04}", 
                rng.gen_range(100..999),
                rng.gen_range(10..99), 
                rng.gen_range(1000..9999)
            ));
        }
        
        if pattern.starts_with("^") && pattern.ends_with("$") {
            // Handle anchored patterns by removing anchors
            let inner_pattern = &pattern[1..pattern.len()-1];
            return self.generate_from_pattern(inner_pattern);
        }
        
        // Generic pattern handling for simple cases
        if let Ok(generated) = self.generate_simple_pattern(pattern, &mut rng) {
            return Ok(generated);
        }
        
        // Fallback: return a string that might match the pattern structure
        Ok(format!("PATTERN_MATCH_{}", rng.gen_range(1000..9999)))
    }

    /// Generate from simple patterns using basic regex interpretation
    fn generate_simple_pattern(&self, pattern: &str, rng: &mut impl Rng) -> Result<String> {
        let mut result = String::new();
        let chars: Vec<char> = pattern.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            match chars[i] {
                '\\' if i + 1 < chars.len() => {
                    match chars[i + 1] {
                        'd' => {
                            result.push_str(&rng.gen_range(0..10).to_string());
                            i += 2;
                        }
                        'w' => {
                            let alphanumeric = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
                            let idx = rng.gen_range(0..alphanumeric.len());
                            result.push(alphanumeric.chars().nth(idx).unwrap());
                            i += 2;
                        }
                        's' => {
                            result.push(' ');
                            i += 2;
                        }
                        _ => {
                            result.push(chars[i + 1]);
                            i += 2;
                        }
                    }
                }
                '[' => {
                    // Find closing bracket and generate from character class
                    let mut j = i + 1;
                    while j < chars.len() && chars[j] != ']' {
                        j += 1;
                    }
                    if j < chars.len() {
                        let char_class: String = chars[i+1..j].iter().collect();
                        if let Some(ch) = self.generate_from_char_class(&char_class, rng) {
                            result.push(ch);
                        }
                        i = j + 1;
                    } else {
                        result.push('[');
                        i += 1;
                    }
                }
                '.' => {
                    // Any character except newline
                    let printable = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()";
                    let idx = rng.gen_range(0..printable.len());
                    result.push(printable.chars().nth(idx).unwrap());
                    i += 1;
                }
                '{' => {
                    // Handle quantifiers like {3}, {2,5}
                    let mut j = i + 1;
                    while j < chars.len() && chars[j] != '}' {
                        j += 1;
                    }
                    if j < chars.len() {
                        let quantifier: String = chars[i+1..j].iter().collect();
                        let count = if let Ok(n) = quantifier.parse::<usize>() {
                            n
                        } else if quantifier.contains(',') {
                            let parts: Vec<&str> = quantifier.split(',').collect();
                            if parts.len() == 2 {
                                let min = parts[0].parse::<usize>().unwrap_or(1);
                                let max = parts[1].parse::<usize>().unwrap_or(min + 2);
                                rng.gen_range(min..=max)
                            } else {
                                1
                            }
                        } else {
                            1
                        };
                        
                        // Generate the previous character/group 'count' times
                        if !result.is_empty() {
                            let last_char = result.chars().last().unwrap();
                            for _ in 1..count {
                                if last_char.is_ascii_digit() {
                                    result.push_str(&rng.gen_range(0..10).to_string());
                                } else if last_char.is_ascii_uppercase() {
                                    let ch = (b'A' + rng.gen_range(0..26)) as char;
                                    result.push(ch);
                                } else if last_char.is_ascii_lowercase() {
                                    let ch = (b'a' + rng.gen_range(0..26)) as char;
                                    result.push(ch);
                                } else {
                                    result.push(last_char);
                                }
                            }
                        }
                        i = j + 1;
                    } else {
                        result.push('{');
                        i += 1;
                    }
                }
                '+' => {
                    // One or more - repeat 1-3 times
                    if !result.is_empty() {
                        let last_char = result.chars().last().unwrap();
                        for _ in 0..rng.gen_range(0..3) {
                            if last_char.is_ascii_digit() {
                                result.push_str(&rng.gen_range(0..10).to_string());
                            } else if last_char.is_ascii_uppercase() {
                                let ch = (b'A' + rng.gen_range(0..26)) as char;
                                result.push(ch);
                            } else if last_char.is_ascii_lowercase() {
                                let ch = (b'a' + rng.gen_range(0..26)) as char;
                                result.push(ch);
                            } else {
                                result.push(last_char);
                            }
                        }
                    }
                    i += 1;
                }
                '*' => {
                    // Zero or more - repeat 0-2 times
                    if !result.is_empty() {
                        let last_char = result.chars().last().unwrap();
                        for _ in 0..rng.gen_range(0..3) {
                            if last_char.is_ascii_digit() {
                                result.push_str(&rng.gen_range(0..10).to_string());
                            } else if last_char.is_ascii_uppercase() {
                                let ch = (b'A' + rng.gen_range(0..26)) as char;
                                result.push(ch);
                            } else if last_char.is_ascii_lowercase() {
                                let ch = (b'a' + rng.gen_range(0..26)) as char;
                                result.push(ch);
                            } else {
                                result.push(last_char);
                            }
                        }
                    }
                    i += 1;
                }
                '?' => {
                    // Zero or one - 50% chance to skip
                    if rng.gen_bool(0.5) && !result.is_empty() {
                        result.pop();
                    }
                    i += 1;
                }
                '|' | '^' | '$' | '(' | ')' => {
                    // Regex metacharacters - skip for now
                    i += 1;
                }
                c => {
                    result.push(c);
                    i += 1;
                }
            }
        }
        
        if result.is_empty() {
            return Err(DataGeneratorError::FieldGeneration(
                format!("Could not generate string from pattern: {pattern}")
            ));
        }
        
        Ok(result)
    }
    
    /// Generate a character from a character class like [a-zA-Z0-9]
    fn generate_from_char_class(&self, char_class: &str, rng: &mut impl Rng) -> Option<char> {
        if char_class.contains("a-z") && char_class.contains("A-Z") && char_class.contains("0-9") {
            let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
            let idx = rng.gen_range(0..chars.len());
            return chars.chars().nth(idx);
        }
        
        if char_class.contains("a-z") {
            let ch = (b'a' + rng.gen_range(0..26)) as char;
            return Some(ch);
        }
        
        if char_class.contains("A-Z") {
            let ch = (b'A' + rng.gen_range(0..26)) as char;
            return Some(ch);
        }
        
        if char_class.contains("0-9") {
            let ch = (b'0' + rng.gen_range(0..10)) as char;
            return Some(ch);
        }
        
        // Fallback: pick first character if available
        char_class.chars().next()
    }

    /// Heuristic-based generation when no pattern is available
    fn generate_heuristic(&self, context: &GenerationContext) -> Result<String> {
        let mut rng = rand::thread_rng();
        
        let property_lower = context.property.to_lowercase();
        
        if property_lower.contains("phone") || property_lower.contains("tel") {
            Ok(format!("{:03}-{:03}-{:04}", 
                rng.gen_range(100..999),
                rng.gen_range(100..999), 
                rng.gen_range(1000..9999)
            ))
        } else if property_lower.contains("email") {
            let domains = ["example.com", "test.org", "sample.edu"];
            let users = ["user", "admin", "test", "demo"];
            Ok(format!("{}{}@{}", 
                users[rng.gen_range(0..users.len())],
                rng.gen_range(10..99),
                domains[rng.gen_range(0..domains.len())]
            ))
        } else if property_lower.contains("url") || property_lower.contains("website") {
            let domains = ["example.com", "test.org", "sample.net"];
            Ok(format!("https://{}", domains[rng.gen_range(0..domains.len())]))
        } else if property_lower.contains("id") || property_lower.contains("identifier") {
            Ok(format!("ID{:06}", rng.gen_range(100000..999999)))
        } else {
            // Generic string generation
            let words = ["Alpha", "Beta", "Gamma", "Delta", "Epsilon"];
            Ok(format!("{}{:03}", 
                words[rng.gen_range(0..words.len())], 
                rng.gen_range(100..999)
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn test_phone_pattern_generation() {
        let generator = PatternGenerator;
        let mut context = GenerationContext::new(
            "http://example.org/phone".to_string(),
            "http://www.w3.org/2001/XMLSchema#string".to_string(),
            "subject1".to_string(),
        );
        context.parameters.insert("pattern".to_string(), json!("\\d{3}-\\d{3}-\\d{4}"));
        
        let result = generator.generate(&context).unwrap();
        println!("Generated phone: {result}");
        
        // Check format: XXX-XXX-XXXX
        let parts: Vec<&str> = result.split('-').collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0].len(), 3);
        assert_eq!(parts[1].len(), 3);
        assert_eq!(parts[2].len(), 4);
    }

    #[test]
    fn test_email_pattern_generation() {
        let generator = PatternGenerator;
        let mut context = GenerationContext::new(
            "http://example.org/email".to_string(),
            "http://www.w3.org/2001/XMLSchema#string".to_string(),
            "subject1".to_string(),
        );
        context.parameters.insert("pattern".to_string(), json!("[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}"));
        
        let result = generator.generate(&context).unwrap();
        println!("Generated email: {result}");
        
        assert!(result.contains("@"));
        assert!(result.contains("."));
    }

    #[test]
    fn test_heuristic_generation() {
        let generator = PatternGenerator;
        let context = GenerationContext::new(
            "http://example.org/phone".to_string(),
            "http://www.w3.org/2001/XMLSchema#string".to_string(),
            "subject1".to_string(),
        );
        
        let result = generator.generate(&context).unwrap();
        println!("Generated heuristic phone: {result}");
        
        // Should generate phone-like format
        assert!(result.contains("-"));
    }
}
