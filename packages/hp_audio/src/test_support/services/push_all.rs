use crate::{AudioReorder, OrderedFrame, SoundPacket};

pub fn push_all(
    reorder: &mut AudioReorder,
    ssrc: u32,
    seqs: impl IntoIterator<Item = u16>,
) -> Vec<OrderedFrame> {
    seqs.into_iter()
        .flat_map(|seq| {
            reorder.push(SoundPacket {
                ssrc,
                seq,
                payload: seq.to_be_bytes().to_vec(),
            })
        })
        .collect()
}
