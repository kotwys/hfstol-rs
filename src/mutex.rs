#[cfg(not(feature = "std"))]
use core::cell::UnsafeCell;

// Given we are single-threaded for now, fake a Mutex
#[cfg(not(feature = "std"))]
pub struct Mutex<T>(UnsafeCell<T>);

#[cfg(not(feature = "std"))]
impl<T> Mutex<T> {
    pub fn new(data: T) -> Self {
        Self(UnsafeCell::new(data))
    }

    pub fn lock(&self) -> Result<&mut T, &'static str> {
        Ok(unsafe { self.0.get() })
    }

    pub fn into_inner(self) -> Result<T, &'static str> {
        Ok(unsafe { self.0.into_inner() })
    }
}

#[cfg(not(feature = "std"))]
unsafe impl<T> Sync for Mutex<T> {}
#[cfg(not(feature = "std"))]
unsafe impl<T> Send for Mutex<T> {}

#[cfg(feature = "std")]
pub use std::sync::Mutex;
