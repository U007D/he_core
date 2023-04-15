use core::{ffi::CStr, ops::Deref};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct CArgs(&'static [&'static CStr]);

impl Deref for CArgs {
    type Target = [&'static CStr];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
