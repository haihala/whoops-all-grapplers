mod pad_stream;
mod parrot_stream;
mod prewritten_stream;
mod test_stream;

pub use pad_stream::{update_pads, PadStream};
pub use parrot_stream::{update_parrots, ParrotStream};
pub use prewritten_stream::PreWrittenStream;
pub use test_stream::TestStream;

use crate::InputEvent;

pub trait InputStream {
    fn read(&mut self) -> Vec<InputEvent>;
}
