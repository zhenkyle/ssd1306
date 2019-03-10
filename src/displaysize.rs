//! Display size

use crate::displaybuffer::{Buffer128x64, DisplayBuffer};

/// Display size
pub trait DisplaySize: Copy {
    /// Get the width and height of this display size
    fn dimensions(&self) -> (u8, u8);

    /// Get a buffer that holds enough 1BPP information for a given display size
    fn buffer(&self) -> impl DisplayBuffer;
}

#[derive(Copy, Clone)]
pub struct Display128x64;

impl DisplaySize for Display128x64 {
    fn dimensions(&self) -> (u8, u8) {
        (128, 64)
    }

    fn buffer(&self) -> Buffer128x64 {
        Buffer128x64::new()
    }
}

// #[derive(Copy, Clone)]
// pub struct Display128x32;

// impl DisplaySize for Display128x32 {
//     fn dimensions(&self) -> (u8, u8) {
//         (128, 32)
//     }

//     fn buffer(&self) -> DB {
//         Buffer128x64::new()
//     }
// }

// #[derive(Copy, Clone)]
// pub struct Display96x16;

// impl DisplaySize for Display96x16 {
//     fn dimensions(&self) -> (u8, u8) {
//         (96, 16)
//     }

//     fn buffer(&self) -> DB {
//         Buffer128x64::new()
//     }
// }

// // TODO: Add to prelude
// /// Display size enumeration
// #[derive(Clone, Copy)]
// pub enum DisplaySize {
//     /// 128 by 64 pixels
//     Display128x64,
//     /// 128 by 32 pixels
//     Display128x32,
//     /// 96 by 16 pixels
//     Display96x16,
// }

// impl DisplaySize {
//     /// Get integral dimensions from DisplaySize
//     // TODO: Use whatever vec2 impl I decide to use here
//     pub fn dimensions(&self) -> (u8, u8) {
//         match *self {
//             DisplaySize::Display128x64 => (128, 64),
//             DisplaySize::Display128x32 => (128, 32),
//             DisplaySize::Display96x16 => (96, 16),
//         }
//     }
// }
