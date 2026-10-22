#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ColourMatrix {
    Identity,
    Bt601,
    Bt709,
    Bt2020,
}
