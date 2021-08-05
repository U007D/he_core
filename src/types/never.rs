#[cfg(feature = "never_trait")]
pub type Never = !;

#[cfg(not(feature = "never_trait"))]
pub enum Never {}
