use crate::DecoderState;

#[test]
fn test_a_new_decoder_state_starts_without_buffered_data() {
    for state in [DecoderState::new(), DecoderState::default()] {
        assert!(
            state.compressed.is_empty(),
            "no compressed bytes are buffered"
        );
        assert!(state.inflated.is_empty(), "no inflated bytes are buffered");
        assert!(state.wire.is_empty(), "no raw bytes are buffered");
        assert_eq!(
            state.zlib_stream.total_in(),
            0,
            "the zlib stream has consumed nothing"
        );
        assert_eq!(
            state.zrle_stream.total_in(),
            0,
            "the ZRLE stream has consumed nothing"
        );
        assert!(
            state
                .tight_streams
                .iter()
                .all(|stream| stream.total_in() == 0),
            "all four Tight streams have consumed nothing"
        );
    }
}
