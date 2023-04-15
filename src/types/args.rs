use core::ops::Deref;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Args(&'static [&'static str]);

impl Deref for Args {
    type Target = [&'static str];

    fn deref(&self) -> &Self::Target { &self.0 }
}
