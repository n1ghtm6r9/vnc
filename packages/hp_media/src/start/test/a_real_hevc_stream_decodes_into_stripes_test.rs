use nvdec::{HevcDecoder, NvdecError};

use crate::{
    MediaChannel, MediaEvent, MediaFactories, MediaStartError, StripeImage, launch_media,
    test_support::{
        BLACK, BLUE, FIRST_DON, FOUR_COLOURS_444, FOUR_COLOURS_HEIGHT, FOUR_COLOURS_WIDTH, RED,
        TestMac, WHITE, access_units, answer_payload, assert_colour_near, event_channel,
        fixed_secrets, message1_payload, next_event, sound_channel, stripe_rgb,
    },
};

#[test]
fn test_a_real_hevc_stream_decodes_into_stripes() {
    let mut mac = TestMac::new();
    let (on_event, events) = event_channel();
    let handle = match launch_media(
        mac.options(),
        on_event,
        MediaFactories {
            new_decoder: HevcDecoder::new,
            new_secrets: fixed_secrets,
            new_sound: sound_channel().0,
        },
    ) {
        Ok(handle) => handle,
        Err(MediaStartError::Decoder(
            error @ (NvdecError::Load(_)
            | NvdecError::Cuda {
                call: "cuInit" | "cuDeviceGet",
                ..
            }),
        )) => {
            println!("no NVIDIA video decoder here: {error}");
            return;
        }
        Err(error) => panic!("the media did not start: {error}"),
    };

    handle.message1(message1_payload());
    next_event(&events, |event| matches!(event, MediaEvent::Offer(_)));
    handle.answer(answer_payload(FOUR_COLOURS_WIDTH, FOUR_COLOURS_HEIGHT, 1));
    for (index, units) in access_units(FOUR_COLOURS_444).iter().enumerate() {
        let datagrams = mac.server.picture(FIRST_DON + index as u16, 0, units);
        mac.send(MediaChannel::Video, &datagrams, &handle);
    }
    let images: Vec<StripeImage> = (0..4)
        .map(
            |_| match next_event(&events, |event| matches!(event, MediaEvent::Stripe(_))) {
                MediaEvent::Stripe(image) => image,
                _ => unreachable!("only stripes are awaited"),
            },
        )
        .collect();

    for image in &images {
        assert_eq!(
            (image.y, image.width, image.height),
            (0, FOUR_COLOURS_WIDTH, FOUR_COLOURS_HEIGHT),
            "each picture fills the single stripe"
        );
    }
    let last_row = FOUR_COLOURS_HEIGHT - 1;
    let last_column = FOUR_COLOURS_WIDTH - 1;
    assert_colour_near(stripe_rgb(&images[0], 0, 0), WHITE, "the first picture");
    assert_colour_near(stripe_rgb(&images[1], 0, 0), BLACK, "the second picture");
    assert_colour_near(
        stripe_rgb(&images[2], 0, 0),
        RED,
        "the top of the third picture",
    );
    assert_colour_near(
        stripe_rgb(&images[2], 0, last_row),
        BLUE,
        "the bottom of the third picture",
    );
    assert_colour_near(
        stripe_rgb(&images[3], 0, 0),
        RED,
        "the left of the fourth picture",
    );
    assert_colour_near(
        stripe_rgb(&images[3], last_column, 0),
        BLUE,
        "the right of the fourth picture",
    );
}
