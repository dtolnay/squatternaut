use serde_derive::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{self, Display};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct CrateName(String);

impl CrateName {
    pub(crate) fn new(string: String) -> Self {
        CrateName(string)
    }
}

impl Display for CrateName {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

impl Ord for CrateName {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0
            .bytes()
            .map(CaseInsensitiveByte)
            .cmp(rhs.0.bytes().map(CaseInsensitiveByte))
    }
}

impl PartialOrd for CrateName {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Eq for CrateName {}

impl PartialEq for CrateName {
    fn eq(&self, rhs: &Self) -> bool {
        <Self as PartialEq<str>>::eq(self, &rhs.0)
    }
}

impl<T> PartialEq<&T> for CrateName
where
    T: ?Sized,
    CrateName: PartialEq<T>,
{
    fn eq(&self, rhs: &&T) -> bool {
        <Self as PartialEq<T>>::eq(self, rhs)
    }
}

impl PartialEq<str> for CrateName {
    fn eq(&self, rhs: &str) -> bool {
        self.0
            .bytes()
            .map(CaseInsensitiveByte)
            .eq(rhs.bytes().map(CaseInsensitiveByte))
    }
}

struct CaseInsensitiveByte(u8);

impl Ord for CaseInsensitiveByte {
    fn cmp(&self, rhs: &Self) -> Ordering {
        let lhs = if self.0 == b'_' {
            b'-'
        } else {
            self.0.to_ascii_lowercase()
        };
        let rhs = if rhs.0 == b'_' {
            b'-'
        } else {
            rhs.0.to_ascii_lowercase()
        };
        lhs.cmp(&rhs)
    }
}

impl PartialOrd for CaseInsensitiveByte {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Eq for CaseInsensitiveByte {}

impl PartialEq for CaseInsensitiveByte {
    fn eq(&self, rhs: &Self) -> bool {
        self.cmp(rhs) == Ordering::Equal
    }
}
