use std::fmt::Debug;

/// Name trait
///
/// Currently Name is equivalent to `AsRef<str> + Debug`.
pub trait Name {
}

impl<T: AsRef<str> + Debug> Name for T {
}
