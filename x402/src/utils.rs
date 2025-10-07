use core::fmt;

pub trait BorrowedStr {
    fn as_ref(&self) -> &str;
}

impl fmt::Debug for &'_ dyn BorrowedStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl PartialEq for &'_ dyn BorrowedStr {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
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
        self.as_ref().cmp(other.as_ref())
    }
}

pub trait OptionalBorrowedStr {
    fn as_ref(&self) -> Option<&str>;
}

impl fmt::Debug for &'_ dyn OptionalBorrowedStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref().unwrap_or("null"))
    }
}

impl PartialEq for &'_ dyn OptionalBorrowedStr {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
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
        self.as_ref().cmp(&other.as_ref())
    }
}
