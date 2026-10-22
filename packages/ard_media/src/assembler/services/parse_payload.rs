use super::nal_type;
use crate::{NAL_TYPE_AGGREGATION, NAL_TYPE_FRAGMENT, ParsedPayload, PayloadPart};

pub(crate) fn parse_payload(payload: &[u8]) -> Option<ParsedPayload> {
    let [first, second, don_high, don_low, rest @ ..] = payload else {
        return None;
    };
    let kind = nal_type(*first);
    let don = u16::from_be_bytes([*don_high, *don_low]);
    if kind < NAL_TYPE_AGGREGATION {
        let mut unit = Vec::with_capacity(payload.len() - 2);
        unit.extend_from_slice(&[*first, *second]);
        unit.extend_from_slice(rest);
        return Some(ParsedPayload {
            don,
            part: PayloadPart::Units(vec![unit]),
        });
    }
    if kind == NAL_TYPE_AGGREGATION {
        let mut units = Vec::new();
        let mut remaining = rest;
        while let [size_high, size_low, after @ ..] = remaining {
            let size = usize::from(u16::from_be_bytes([*size_high, *size_low]));
            if size < 2 || size > after.len() {
                return None;
            }
            let (unit, tail) = after.split_at(size);
            units.push(unit.to_vec());
            remaining = tail;
        }
        if !remaining.is_empty() || units.is_empty() {
            return None;
        }
        return Some(ParsedPayload {
            don,
            part: PayloadPart::Units(units),
        });
    }
    if kind == NAL_TYPE_FRAGMENT {
        let [fragment_header, don_high, don_low, data @ ..] = &payload[2..] else {
            return None;
        };
        return Some(ParsedPayload {
            don: u16::from_be_bytes([*don_high, *don_low]),
            part: PayloadPart::Fragment {
                header: [*first, *second, *fragment_header],
                data: data.to_vec(),
            },
        });
    }
    None
}
