use std::cmp::Ordering;

use core::MoveId;

#[derive(Debug)]
pub(super) struct MoveActivation {
    pub kind: ActivationType,
    pub id: MoveId,
}

#[derive(Debug)]
pub(super) enum ActivationType {
    Continuation,
    Raw,
    Link(Link),
    Cancel(Cancellation),
}

// +- frames. 0 is frame perfect, 1 means +-1 aka 3 frame window
const PERFECT_TIMING_DELTA: i32 = 0;
const GOOD_TIMING_DELTA: i32 = 3;

#[derive(Debug)]
enum ErrorDirection {
    Late,
    Early,
}
impl From<i32> for ErrorDirection {
    fn from(error: i32) -> Self {
        if error.signum() == 1 {
            Self::Late
        } else {
            Self::Early
        }
    }
}

#[derive(Debug)]
enum LinkPrecision {
    Perfect,
    Good(ErrorDirection),
    Fine(ErrorDirection),
}
impl LinkPrecision {
    fn from_frame_diff(frame_diff: i32) -> Self {
        if frame_diff.abs() <= PERFECT_TIMING_DELTA {
            Self::Perfect
        } else if frame_diff.abs() <= GOOD_TIMING_DELTA {
            Self::Good(frame_diff.into())
        } else {
            Self::Fine(frame_diff.into())
        }
    }

    fn meter_gain(&self) -> Option<i32> {
        match self {
            LinkPrecision::Perfect => Some(30),
            LinkPrecision::Good(_) => Some(10),
            LinkPrecision::Fine(_) => None,
        }
    }

    fn message(&self) -> String {
        match self {
            LinkPrecision::Perfect => "Perfect link!".to_owned(),
            LinkPrecision::Good(error) => format!("Good link! ({:?})", error),
            LinkPrecision::Fine(error) => format!("Linked ({:?})", error),
        }
    }
}

#[derive(Debug)]
pub(super) struct Link {
    pub correction: usize,
    precision: LinkPrecision,
}
impl Link {
    pub(super) fn new(stored_frame: usize, freedom_frame: usize) -> Self {
        let error = stored_frame as i32 - freedom_frame as i32;

        Self {
            correction: freedom_frame,
            precision: LinkPrecision::from_frame_diff(error),
        }
    }

    pub(super) fn meter_gain(&self) -> Option<i32> {
        self.precision.meter_gain()
    }

    pub(super) fn message(&self) -> String {
        self.precision.message()
    }
}

#[derive(Debug)]
pub(super) struct Cancellation {
    pub message: String,
}
impl Cancellation {
    pub(super) fn new(input_frame: usize, cancellable_since: usize) -> Self {
        Self {
            message: match input_frame.cmp(&cancellable_since) {
                Ordering::Equal => "Frame perfect cancel".to_owned(),
                Ordering::Greater => {
                    // Input frame came after it was cancellable
                    format!("Cancelled on frame {}", input_frame - cancellable_since)
                }
                Ordering::Less => format!(
                    // Input frame came before it was cancellable
                    "Cancel buffered for {} frames",
                    cancellable_since - input_frame
                ),
            },
        }
    }
}
