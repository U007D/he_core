use alloc::string::String;
use core::ops::Deref;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Args(&'static [String]);

impl Deref for Args {
    type Target = [String];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}