//! src/domain/subscriber_name.rs

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    /// Returns an instance of `SubscriberName` if the input satisfies all
    /// our validation constraints on subscriber names.
    /// It panics otherwise.
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        // `.trim()` returns a view over the input `s` without trailing
        // whitespace like characters.
        // `.is_empty()` checks if the view contains any character.
        let is_empty_or_whitespace = s.trim().is_empty();

        // A grapheme is defined by the Unicode standard as a "user-perceived"
        // character: `å` is a single grapheme, but it is composed of two characters
        // (`a` and ``̊`).
        //
        // `graphemes` returns an iterator over the graphemes in the input `name`.
        // `true` specifies that we want to use the extended grapheme definition set,
        // the recommended one.
        let is_too_long = s.graphemes(true).count() > 256;

        // Iterate over all the characters in the input `name` to check if any of them matches
        // one of the characters in the forbidden array
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|c| forbidden_characters.contains(&c));

        // Return `false` if any of our conditions has been violated
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }
}

// Implement AsRef to enable accessing the inner contents of SubscriberName
// without providing a mutable reference to the contents.
impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ä".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_gets_rejected() {
        let name = "r".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = "\t\n  \t \t \n".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_string_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn invalid_characters_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '{', '}', '\\'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn valid_names_pass() {
        for name in &[
            "Matt LeBlanc",
            "Ursula Le Guin",
            "Johathan Brower Distinct the Fourth",
        ] {
            let name = name.to_string();
            assert_ok!(SubscriberName::parse(name));
        }
    }
}
