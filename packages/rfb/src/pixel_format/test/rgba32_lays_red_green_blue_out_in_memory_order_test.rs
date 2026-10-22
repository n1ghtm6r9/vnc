use crate::PixelFormat;

#[test]
fn test_rgba32_lays_red_green_blue_out_in_memory_order() {
    let format = PixelFormat::rgba32();
    let pixel: u32 = (0x11 << format.red_shift) | (0x22 << format.green_shift) | (0x33 << format.blue_shift);
    assert!(!format.big_endian, "little endian keeps the low byte first in memory");
    assert_eq!(pixel.to_le_bytes(), [0x11, 0x22, 0x33, 0x00], "red, green and blue land in memory as an RGBA texture expects them");
    assert_eq!(format.bytes_per_pixel(), 4, "32 bits are 4 bytes");
}
