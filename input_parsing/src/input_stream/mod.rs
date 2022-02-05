mod pad_stream;
mod test_stream;

pub use pad_stream::{update_pads, PadStream};
pub use test_stream::TestStream;

use crate::helper_types::Diff;

pub trait InputStream {
    fn read(&mut self) -> Option<Diff>;
}
