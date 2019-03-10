//! Abstraction of different operating modes for the SSD1306

use crate::displaysize::DisplaySize;
use crate::interface::DisplayInterface;
use crate::properties::DisplayProperties;

/// Display mode abstraction
pub struct DisplayMode<MODE>(pub MODE);

/// Trait with core functionality for display mode switching
pub trait DisplayModeTrait<DI, DS> {
    /// Allocate all required data and initialise display for mode
    fn new(properties: DisplayProperties<DI, DS>) -> Self;

    /// Release resources for reuse with different mode
    fn release(self) -> DisplayProperties<DI, DS>;
}

impl<MODE> DisplayMode<MODE> {
    /// Setup display to run in requested mode
    pub fn new<DI, DS>(properties: DisplayProperties<DI, DS>) -> Self
    where
        DI: DisplayInterface,
        DS: DisplaySize,
        MODE: DisplayModeTrait<DI, DS>,
    {
        DisplayMode(MODE::new(properties))
    }

    /// Change into any mode implementing DisplayModeTrait
    // TODO: Figure out how to stay as generic DisplayMode but act as particular mode
    pub fn into<DI, DS, NMODE: DisplayModeTrait<DI, DS>>(self) -> NMODE
    where
        DI: DisplayInterface,
        DS: DisplaySize,
        MODE: DisplayModeTrait<DI, DS>,
    {
        let properties = self.0.release();
        NMODE::new(properties)
    }
}
