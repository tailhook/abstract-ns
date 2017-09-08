use std::fmt;
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

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
struct Impl {
    host: String,
    default_port: Option<u16>,
}

/// A name that can be resolved into an address
///
/// Create a name with `Name::from_str`
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
pub struct Name(Arc<Impl>);

impl Name {
    /// Returns hostname part of the name
    pub fn host(&self) -> &str {
        &self.0.host
    }
    /// Returns default port
    pub fn default_port(&self) -> Option<u16> {
        self.0.default_port
    }
}

impl FromStr for Name {
    type Err = Error;
    fn from_str(value: &str) -> Result<Name, Error> {
        let mut pair = value.splitn(2, ':');
        let name = pair.next().unwrap();
        let port = match pair.next() {
            Some(port_str) => {
                Some(port_str.parse().map_err(ErrorEnum::InvalidPort)?)
            }
            None => None,
        };
        namecheck(name)?;
        Ok(Name(Arc::new(Impl {
            host: name.to_string(),
            default_port: port,
        })))
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
                (c.is_lowercase() || c == '-' || c == '_'))
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
        match self.0.default_port {
            Some(p) => write!(f, "{}:{}", self.0.host, p),
            None => f.write_str(&self.0.host),
        }
    }
}


#[cfg(test)]
mod test {
    use std::str::FromStr;
    use std::sync::Arc;
    use super::{Name, Impl};

    fn bare(host: &str) -> Name {
        Name(Arc::new(Impl {
            host: host.to_string(),
            default_port: None,
        }))
    }

    fn full(host: &str, port: u16) -> Name {
        Name(Arc::new(Impl {
            host: host.to_string(),
            default_port: Some(port),
        }))
    }

    fn name_str(src: &str) -> Name {
        Name::from_str(src).unwrap()
    }
    fn name_err(src: &str) -> String {
        Name::from_str(src).unwrap_err().to_string()
    }

    #[test]
    fn bare_name() {
        assert_eq!(name_str("localhost"), bare("localhost"));
        assert_eq!(name_str("host.name.org"), bare("host.name.org"));
        assert_eq!(name_str("name.root."), bare("name.root."));
    }

    #[test]
    fn display() {
        assert_eq!(bare("localhost").to_string(), "localhost");
        assert_eq!(bare("name.example.org.").to_string(), "name.example.org.");
        assert_eq!(bare("name.example.org").to_string(), "name.example.org");
        assert_eq!(full("name", 123).to_string(), "name:123");
        assert_eq!(full("name.org", 2354).to_string(), "name.org:2354");
        assert_eq!(full("name.org.", 2354).to_string(), "name.org.:2354");
    }

    #[test]
    fn host() {
        assert_eq!(bare("localhost").host(), "localhost");
        assert_eq!(bare("name.example.org.").host(), "name.example.org.");
        assert_eq!(bare("name.example.org").host(), "name.example.org");
        assert_eq!(full("name", 123).host(), "name");
        assert_eq!(full("name.org", 2354).host(), "name.org");
        assert_eq!(full("name.org.", 2354).host(), "name.org.");
    }

    #[test]
    fn port() {
        assert_eq!(bare("localhost").default_port(), None);
        assert_eq!(bare("name.example.org.").default_port(), None);
        assert_eq!(bare("name.example.org").default_port(), None);
        assert_eq!(full("name", 123).default_port(), Some(123));
        assert_eq!(full("name.org", 2354).default_port(), Some(2354));
        assert_eq!(full("name.org.", 2354).default_port(), Some(2354));
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
