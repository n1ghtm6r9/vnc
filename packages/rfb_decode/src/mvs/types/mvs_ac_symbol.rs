#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MvsAcSymbol {
    Value(i32),
    Skip(usize),
    EndOfBlock,
}
