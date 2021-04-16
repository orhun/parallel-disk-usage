use crate::size::Size;

/// Scan progress.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProgressReport<Data: Size> {
    /// Number of known items.
    pub known_items: u64,
    /// Number of scanned items.
    pub scanned_items: u64,
    /// Total size of scanned items.
    pub scanned_total: Data,
    /// Number of occurred errors.
    pub errors: u64,
}