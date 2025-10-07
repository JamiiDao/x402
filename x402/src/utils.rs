use core::fmt;
use std::borrow::Cow;

pub trait BorrowedStr {
    fn as_str(&self) -> &str;
}
impl fmt::Debug for &'_ dyn BorrowedStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl PartialEq for &'_ dyn BorrowedStr {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for &'_ dyn BorrowedStr {}

impl PartialOrd for &'_ dyn BorrowedStr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for &'_ dyn BorrowedStr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

pub trait OptionalBorrowedStr {
    fn as_str(&self) -> Option<&str>;
}

impl fmt::Debug for &'_ dyn OptionalBorrowedStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str().unwrap_or("null"))
    }
}

impl PartialEq for &'_ dyn OptionalBorrowedStr {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for &'_ dyn OptionalBorrowedStr {}

impl PartialOrd for &'_ dyn OptionalBorrowedStr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for &'_ dyn OptionalBorrowedStr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(&other.as_str())
    }
}

impl BorrowedStr for &str {
    fn as_str(&self) -> &str {
        self
    }
}

impl BorrowedStr for String {
    fn as_str(&self) -> &str {
        self.as_str()
    }
}

impl BorrowedStr for Cow<'_, str> {
    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl OptionalBorrowedStr for &str {
    fn as_str(&self) -> Option<&str> {
        Some(self)
    }
}

impl OptionalBorrowedStr for String {
    fn as_str(&self) -> Option<&str> {
        Some(self.as_str())
    }
}

impl OptionalBorrowedStr for Cow<'_, str> {
    fn as_str(&self) -> Option<&str> {
        Some(self.as_ref())
    }
}
