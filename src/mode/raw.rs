//! Raw mode for coercion into richer driver types
//!
//! A display driver instance without high level functionality used as a return type from the
//! builder. Used as a source to coerce the driver into richer modes like
//! [`GraphicsMode`](../graphics/index.html) and [`TerminalMode`](../terminal/index.html).

use crate::displaysize::DisplaySize;
use crate::interface::DisplayInterface;
use crate::mode::displaymode::DisplayModeTrait;
use crate::properties::DisplayProperties;

/// Raw display mode
pub struct RawMode<DI, DS>
where
    DI: DisplayInterface,
    DS: DisplaySize,
{
    properties: DisplayProperties<DI, DS>,
}

impl<DI, DS> DisplayModeTrait<DI, DS> for RawMode<DI, DS>
where
    DI: DisplayInterface,
    DS: DisplaySize,
{
    /// Create new RawMode instance
    fn new(properties: DisplayProperties<DI, DS>) -> Self {
        RawMode { properties }
    }

    /// Release all resources used by RawMode
    fn release(self) -> DisplayProperties<DI, DS> {
        self.properties
    }
}

impl<DI, DS> RawMode<DI, DS>
where
    DI: DisplayInterface,
    DS: DisplaySize,
{
    /// Create a new raw display mode
    pub fn new(properties: DisplayProperties<DI, DS>) -> Self {
        RawMode { properties }
    }
}
