//! ISO 639 language codes
//!
//! When dealing with different language inputs and APIs, different standards are used to identify
//! a language. Converting between these in an automated way can be tedious. This crate provides an
//! enum which supports conversion from 639-1 and 639-3 and also into these formats, as well as
//! into English names.
//!
//! The language table is compiled into the library. While this increases the binary size, it means
//! that no additional time is wasted on program startup or on table access for allocating or
//! filling the map. It is hence suitable for retrieval of codes in constraint environments.
//!
//! # Examples
//!
//! ```
//! use isolang::Language;
//!
//! assert_eq!(Language::from_639_1("de").unwrap().to_name(), "German");
//! assert_eq!(Language::from_639_3("spa").unwrap().to_639_1(), Some("es"));
//! ```

#[cfg(feature = "serde_serialize")]
extern crate serde;

#[cfg(feature = "serde_serialize")]
mod serde_impl;

extern crate phf;

use std::str;

include!(concat!(env!("OUT_DIR"), "/isotable.rs"));

impl Language {
    /// Create string representation of this Language as a ISO 639-3 code.
    ///
    /// This method will return the ISO 639-3 code, which consists of three letters.
    ///
    /// # Example
    ///
    /// ```
    /// use isolang::Language;
    ///
    /// assert_eq!(Language::Deu.to_639_3(), "deu");
    /// ```
    pub fn to_639_3(&self) -> &'static str {
        // It's safe to do so, we have written that by hand as UTF-8 into the binary and if you
        // haven't changed the binary, it's UTF-8
        unsafe { str::from_utf8_unchecked(&OVERVIEW[*self as usize].0) }
    }

    /// Create two-letter ISO 639-1 representation of the language.
    ///
    /// This will return a two-letter ISO 639-1 code, if it exists and None otherwise.
    /// ISO 639-1 codes are only used for the most common languages.
    ///
    /// # Example
    ///
    /// ```
    /// use isolang::Language;
    ///
    /// assert!(Language::Gha.to_639_1().is_none());
    /// ```
    pub fn to_639_1(&self) -> Option<&'static str> {
        unsafe {
            // Is safe, see `to_639_3()` for more details
            OVERVIEW[*self as usize].1
                .map(|ref s| str::from_utf8_unchecked(*s))
        }
    }

    /// Get the English name of this language.
    ///
    /// This returns the English name of the language, as defined in the ISO 639 standard. It does
    /// not include additional comments, e.g. classification of a macrolanguage, etc.
    ///
    /// # Examples
    ///
    /// ```
    /// use isolang::Language;
    ///
    /// assert_eq!(Language::Spa.to_name(), "Spanish");
    /// // macro language
    /// assert_eq!(Language::Swa.to_name(), "Swahili");
    /// // individual language
    /// assert_eq!(Language::Swh.to_name(), "Swahili");
    /// ```
    pub fn to_name(&self) -> &'static str {
        unsafe {
            // Is safe, see `to_639_3()` for more details
            str::from_utf8_unchecked(OVERVIEW[*self as usize].2)
        }
    }

    /// Create a Language instance rom a ISO 639-1 code.
    ///
    /// This will return a Language instance if the given string is a valid two-letter language
    /// code. For invalid inputs, None is returned.
    ///
    /// # Example
    ///
    /// ```
    /// use isolang::Language;
    ///
    /// assert!(Language::from_639_1("de").is_some());
    /// assert!(Language::from_639_1("…").is_none());
    /// ```
    pub fn from_639_1(code: &str) -> Option<Language> {
        if code.len() != 2 {
            return None;
        }
        TWO_TO_THREE.get(code).cloned()
    }

    /// Create a Language instance rom a ISO 639-3 code.
    ///
    /// This will return a Language instance if the given string is a valid three-letter language
    /// code. For invalid inputs, None is returned.
    ///
    /// # Example
    ///
    /// ```
    /// use isolang::Language;
    ///
    /// assert!(Language::from_639_3("dan").is_some());
    /// assert!(Language::from_639_1("…").is_none());
    /// ```
    pub fn from_639_3(code: &str) -> Option<Language> {
        if code.len() != 3 {
            return None;
        }
        THREE_TO_THREE.get(code).cloned()
    }

    /// Parse language from given locale
    ///
    /// This parses a language from a given locale string, as used by UNIX-alike and other systems.
    ///
    /// # Example
    ///
    /// ```
    /// use isolang::Language;
    ///
    /// fn main() {
    ///     assert!(Language::from_locale("de_DE.UTF-8") == Some(Language::Deu));
    /// }
    /// ```
    pub fn from_locale(locale: &str) -> Option<Language> {
        if locale.len() < 3 {
            return None;
        }
        let mut split = locale.split('_');
        match split.next() {
            Some(letters) => Language::from_639_1(letters),
            None => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    extern crate serde_json;

    #[test]
    fn invalid_locale_gives_none() {
        assert!(Language::from_locale("foo").is_none());
        assert!(Language::from_locale("deu_DEU.UTF-8").is_none());
        assert!(Language::from_locale("___").is_none());
        assert!(Language::from_locale("ää_öö.UTF-8").is_none());
    }

    #[test]
    fn test_valid_locales_are_correctly_decoded() {
        assert!(Language::from_locale("de_DE.UTF-8").unwrap() == Language::Deu);
        assert!(Language::from_locale("en_GB.UTF-8").unwrap() == Language::Eng);
    }

    #[test]
    #[cfg(feature = "serde_serialize")]
    fn test_serde() {
        assert!(serde_json::to_string(&Language::Deu).unwrap() == String::from("\"deu\""));
        assert!(serde_json::from_str::<Language>("\"deu\"").unwrap() == Language::Deu);

        assert!(serde_json::from_str::<Language>("\"foo\"").is_err());
    }
}

impl std::fmt::Debug for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_639_3())
    }
}
