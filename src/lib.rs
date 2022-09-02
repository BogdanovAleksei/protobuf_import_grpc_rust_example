mod base;
mod message_a;
mod message_b;


#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Name(String);

impl From<base::Name> for Name {
    fn from(name: base::Name) -> Self {
        Self(name.name)
    }
}