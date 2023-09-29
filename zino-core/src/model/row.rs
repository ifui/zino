/// A collection of values that can be decoded from a single row.
pub trait DecodeRow<Row>: Default + Sized {
    /// The error type.
    type Error;

    /// Decodes a row and attempts to create an instance of `Self`.
    fn decode_row(row: &Row) -> Result<Self, Self::Error>;
}
