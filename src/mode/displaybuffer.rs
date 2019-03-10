//! Display buffers for different sizes of display

use core::ops::{Deref, DerefMut};

/// 1024 byte buffer for 128x64px displays, 1BPP
#[derive(Copy, Clone)]
pub struct Buffer128x64(pub [u8; 1024]);

/// 512 byte buffer for 128x32px displays, 1BPP
#[derive(Copy, Clone)]
pub struct Buffer128x32(pub [u8; 512]);

/// 192 byte buffer for 96x16px displays, 1BPP
#[derive(Copy, Clone)]
pub struct Buffer96x16(pub [u8; 192]);

/// Display buffer trait
pub trait DisplayBuffer: Sized + Deref<Target = [u8]> + DerefMut {
    /// Create a new, empty buffer
    fn new() -> Self;

    /// Clear the buffer
    fn clear(&mut self) -> Self {
        Self::new()
    }
}

impl DisplayBuffer for Buffer128x64 {
    fn new() -> Self {
        Self([0; 1024])
    }
}

impl Deref for Buffer128x64 {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl DerefMut for Buffer128x64 {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl DisplayBuffer for Buffer128x32 {
    fn new() -> Self {
        Self([0; 512])
    }
}

impl Deref for Buffer128x32 {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl DerefMut for Buffer128x32 {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl DisplayBuffer for Buffer96x16 {
    fn new() -> Self {
        Self([0; 192])
    }
}

impl Deref for Buffer96x16 {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl DerefMut for Buffer96x16 {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}
