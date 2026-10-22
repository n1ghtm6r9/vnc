use ard_media::ProtoField;

use super::{answer_stream, with_length};
use crate::test_support::{TEST_AUDIO_SSRC, TEST_VIDEO_SSRC};

pub fn answer_payload(width: u32, height: u32, stripes: u32) -> Vec<u8> {
    let audio = answer_stream(&[
        ProtoField::varint(1, 1),
        ProtoField::message(
            3,
            &[
                ProtoField::varint(1, TEST_AUDIO_SSRC.into()),
                ProtoField::varint(6, 1),
            ],
        ),
        ProtoField::text(6, "Viceroy 1.7.0"),
    ]);
    let video = answer_stream(&[
        ProtoField::varint(1, 1),
        ProtoField::message(
            5,
            &[
                ProtoField::varint(1, TEST_VIDEO_SSRC.into()),
                ProtoField::varint(4, width.into()),
                ProtoField::varint(5, height.into()),
                ProtoField::varint(6, stripes.into()),
            ],
        ),
    ]);
    let mut body = vec![0, 2, 0, 2, 0, 0, 0, 1];
    body.extend_from_slice(&(audio.len() as u16).to_be_bytes());
    body.extend_from_slice(&(video.len() as u16).to_be_bytes());
    body.extend_from_slice(&[0; 6]);
    body.extend(audio);
    body.extend(video);
    with_length(body)
}
