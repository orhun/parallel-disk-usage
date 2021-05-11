use strum::{AsRefStr, EnumString, EnumVariantNames, VariantNames};

/// Quantity to be measured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsRefStr, EnumString, EnumVariantNames)]
pub enum Quantity {
    /// Measure apparent sizes, equivalent to the
    /// [len](std::fs::Metadata::len) method.
    #[strum(serialize = "len")]
    ApparentSize,
    /// Measure block sizes, equivalent to the
    /// [blksize](std::os::unix::prelude::MetadataExt::blksize) method (POSIX only).
    #[cfg(unix)]
    #[strum(serialize = "blksize")]
    BlockSize,
    /// Count numbers of blocks, equivalent to the
    /// [blocks](std::os::unix::prelude::MetadataExt::blocks) method (POSIX only).
    #[cfg(unix)]
    #[strum(serialize = "blocks")]
    BlockCount,
}

/// Possible CLI values of [`Quantity`].
pub const QUANTITY_VALUES: &[&str] = Quantity::VARIANTS;

impl Quantity {
    pub(super) fn default_value() -> &'static str {
        Quantity::ApparentSize.as_ref()
    }
}
