/// Result type of gc
pub type GcResult<T> = std::result::Result<T, GcError>;

/// Error type for the GC
#[derive(Debug, Copy, Clone)]
pub enum GcError {
    /// The object is not a valid gc object.
    UnknownError
}


