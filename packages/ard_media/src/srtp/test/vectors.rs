use crate::{MasterKey, test_support::hex_bytes};

pub(super) fn reference_master() -> MasterKey {
    MasterKey {
        key: std::array::from_fn(|index| index as u8),
        salt: std::array::from_fn(|index| 0xa0 + index as u8),
    }
}

pub(super) const EXTENSION: &str = "9311000100030007";

pub(super) const RTP_VECTORS: [(&str, &str, &str); 4] = [
    (
        "no extension",
        "806412340001020311223344303132333435363738393a3b3c3d3e3f40414243",
        "8064123400010203112233445f4032dd3cb990dc23907f5dded8392babd46d829e116069d41f8cb8b3dd",
    ),
    (
        "extension and marker",
        "90e40042fedcba98deadbeef9311000100030007030a11181f262d343b424950575e656c737a81888f96\
         9da4abb2b9c0c7ced5dce3eaf1f8ff",
        "90e40042fedcba98deadbeef9311000100030007c9317c018f616e695c38b99fbbfbd1b62bd9b94dc5aa\
         67c482b280c1c47b535108c1dda93a23f3cc39fedcd6c90ebf",
    ),
    (
        "seq 65535, rollover counter 0",
        "9064ffff00002ee00badcafe931100010003000700010409101924314051647990a9c4e1",
        "9064ffff00002ee00badcafe93110001000300071011a946ebf7daf15f1324ff341d3a7ecfba81497160\
         eb818984",
    ),
    (
        "seq 0, rollover counter 1",
        "90e4000000002ee00badcafe9311000100030007fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0\
         bdbab7b4b1aeaba8a5a29f",
        "90e4000000002ee00badcafe931100010003000795097496397d2a5b95d9ab49f46a3a34a45694d4c77f\
         73b7574473a907903cc43d04cfc3dcf2b4015d3001",
    ),
];

pub(super) const RTCP_VECTORS: [(&str, &str, &str); 2] = [
    (
        "receiver report and source description, index 1",
        "81c900070a0b0c0d11223344000000000000123400000000000000000000000081ca00020a0b0c0d0100\
         0000",
        "81c900070a0b0c0d750c6119dd9174e387108b4c8fdc8f6f24030af2b97a69b63dff6c1338531c49e31d\
         ff81800000010b45224572b32ecb2b88",
    ),
    (
        "empty receiver report, index 2",
        "80c900010a0b0c0d",
        "80c900010a0b0c0d800000028292dc86ed371a70208e",
    ),
];

pub(super) fn bytes(text: &str) -> Vec<u8> {
    hex_bytes(text)
}
