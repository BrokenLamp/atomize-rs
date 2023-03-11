use std::{
    collections::hash_map::DefaultHasher,
    fmt,
    hash::{Hash, Hasher},
    ops::Add,
};

/// Elixir style atom macro
/// ```rs
/// a!(apple) == a!(apple)
/// a!(apple) != a!(orange)
/// ```
pub use atomize_macro::a;

#[derive(Debug, Copy, Clone)]
pub struct Atom {
    value: u64,
}

impl Atom {
    pub fn value(&self) -> u64 {
        self.value
    }
}

impl Atom {
    pub fn mix(&self, other: &Atom) -> Self {
        Atom {
            value: self.value ^ other.value,
        }
    }
}

impl Add for Atom {
    type Output = Atom;
    fn add(self, other: Atom) -> <Self as Add<Atom>>::Output {
        Atom::mix(&self, &other)
    }
}

impl From<u64> for Atom {
    fn from(value: u64) -> Self {
        Atom { value }
    }
}

impl From<&str> for Atom {
    fn from(name: &str) -> Self {
        let value: u64 = {
            let mut hasher = DefaultHasher::new();
            name.hash(&mut hasher);
            hasher.finish()
        };
        Atom { value }
    }
}

impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<u64> for Atom {
    fn eq(&self, other: &u64) -> bool {
        self.value == *other
    }
}

impl PartialEq<str> for Atom {
    fn eq(&self, other: &str) -> bool {
        let mut hasher = DefaultHasher::new();
        other.hash(&mut hasher);
        self.value == hasher.finish()
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a:{:016x}", self.value)
    }
}
