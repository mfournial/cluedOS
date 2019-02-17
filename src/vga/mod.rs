use volatile::Volatile;

mod colours;
mod writer;

// Exposes print macros
pub use writer::macros;
pub use writer::_print;

// VGA output size
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer {
    chars: [[Volatile<writer::ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
