//! Name type and helper types
//!
use std::fmt;
#[allow(unused_imports, deprecated)]
use std::ascii::AsciiExt;
use std::str::FromStr;
use std::num::ParseIntError;
use std::sync::Arc;

quick_error! {
    /// Error parsing Name from string
    #[derive(Debug)]
    pub enum Error wraps ErrorEnum {
        DotError {
            description("name can't start with dot and \
                can't have subsequent dots")
        }
        InvalidChar {
            description("only ascii numbers and letters, \
                dash `-`, underscore `_` and dot `.` are supported in names")
        }
        InvalidPrefixSuffix {
            description("any part of name can't start or end with dash")
        }
        InvalidPort(err: ParseIntError) {
             description("error parsing default port number")
             display("default port number is invalid: {}", err)
             from()
        }
    }
}

/// A name is a barely ``Arc<String>`` but also checks that name is valid
///
/// Note: this is designed to be static, because it's often used inside
/// the futures which can't contain non-static content.
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
pub struct Name(Arc<str>);

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Name {
    /// Create a name from an Arc.
    ///
    /// This allows to keep Arc shared with
    /// other components of your application
    pub fn from_arc(arc: &Arc<str>) -> Result<Name, Error> {
        namecheck(arc)?;
        Ok(Name(arc.clone()))
    }
    /// Return a clone of the inner Arc
    ///
    /// This allows to keep Arc shared with
    /// other components of your application
    pub fn inner(&self) -> Arc<str> {
        self.0.clone()
    }
}

impl FromStr for Name {
    type Err = Error;
    fn from_str(value: &str) -> Result<Name, Error> {
        namecheck(value)?;
        Ok(Name(value.into()))
    }
}

fn namecheck(mut name: &str) -> Result<(), Error> {
    // The dot at the end is allowed (means don't add search domain)
    if name.ends_with('.') {
        name = &name[..name.len()-1];
    }
    let pieces = name.split('.');
    for piece in pieces {
        if piece.len() == 0 {
            return Err(ErrorEnum::DotError.into());
        }
        if !piece.chars()
            .all(|c| c.is_ascii() &&
                (c.is_lowercase() || c.is_numeric() || c == '-' || c == '_'))
        {
            return Err(ErrorEnum::InvalidChar.into());
        }
        if piece.starts_with("-") || piece.ends_with("-") {
            return Err(ErrorEnum::InvalidPrefixSuffix.into());
        }
    }
    Ok(())
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}


#[cfg(test)]
mod test {
    use std::str::FromStr;
    use super::Name;

    fn name_str(src: &str) -> Name {
        Name::from_str(src).unwrap()
    }
    fn name_err(src: &str) -> String {
        Name::from_str(src).unwrap_err().to_string()
    }
    fn bare(name: &str) -> Name {
        Name(name.into())
    }

    #[test]
    fn bare_name() {
        assert_eq!(name_str("localhost"), bare("localhost"));
        assert_eq!(name_str("host.name.org"), bare("host.name.org"));
        assert_eq!(name_str("name.root."), bare("name.root."));
    }

    #[test]
    fn name_with_numbers() {
        assert_eq!(name_str("x1"), bare("x1"));
        assert_eq!(name_str("x1.y1"), bare("x1.y1"));
        assert_eq!(name_str("1.2.x"), bare("1.2.x"));
    }

    #[test]
    fn name_with_dashes() {
        assert_eq!(name_str("x-a"), bare("x-a"));
        assert_eq!(name_str("x-a.y-b"), bare("x-a.y-b"));
    }

    #[test]
    fn display() {
        assert_eq!(bare("localhost").to_string(), "localhost");
        assert_eq!(bare("name.example.org.").to_string(), "name.example.org.");
        assert_eq!(bare("name.example.org").to_string(), "name.example.org");
    }

    #[test]
    fn dash() {
        assert_eq!(name_err("-name"),
            "any part of name can\'t start or end with dash");
        assert_eq!(name_err("name-"),
            "any part of name can\'t start or end with dash");
        assert_eq!(name_err("x.-y"),
            "any part of name can\'t start or end with dash");
        assert_eq!(name_err("x-.y"),
            "any part of name can\'t start or end with dash");
        assert_eq!(name_err("x-.-y"),
            "any part of name can\'t start or end with dash");
        assert_eq!(name_err("-xx.yy-"),
            "any part of name can\'t start or end with dash");
    }

    #[test]
    fn two_dots() {
        assert_eq!(name_err("name..name"),
            "name can\'t start with dot and can\'t have subsequent dots");
        assert_eq!(name_err(".name.org"),
            "name can\'t start with dot and can\'t have subsequent dots");
        assert_eq!(name_err("..name.org"),
            "name can\'t start with dot and can\'t have subsequent dots");
        assert_eq!(name_err("name.org.."),
            "name can\'t start with dot and can\'t have subsequent dots");
    }
}
