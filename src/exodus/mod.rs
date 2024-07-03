#[cfg(feature = "python")]
pub mod py;

pub struct Exodus {}

impl Exodus {
    pub fn init() -> Self {
        Self {}
    }
}
