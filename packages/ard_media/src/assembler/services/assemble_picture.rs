use std::collections::HashMap;

use super::{is_irap, nal_type};
use crate::{ANNEX_B_START_CODE, PayloadPart, seq_delta};

pub(crate) fn assemble_picture(parts: &HashMap<u16, PayloadPart>) -> Option<(Vec<u8>, bool)> {
    let reference = i64::from(*parts.keys().next()?);
    let mut ordered: Vec<(i64, &PayloadPart)> = parts
        .iter()
        .map(|(seq, part)| (seq_delta(i64::from(*seq), reference), part))
        .collect();
    ordered.sort_unstable_by_key(|(delta, _)| *delta);
    let size: usize = ordered
        .iter()
        .map(|(_, part)| match part {
            PayloadPart::Units(units) => units.iter().map(|unit| unit.len() + 4).sum(),
            PayloadPart::Fragment { data, .. } => data.len() + 6,
        })
        .sum();
    let mut data = Vec::with_capacity(size);
    let mut key = false;
    let mut open = None;
    for (_, part) in ordered {
        match part {
            PayloadPart::Units(units) => {
                if open.is_some() {
                    return None;
                }
                for unit in units {
                    data.extend_from_slice(&ANNEX_B_START_CODE);
                    data.extend_from_slice(unit);
                    key |= is_irap(nal_type(unit[0]));
                }
            }
            PayloadPart::Fragment {
                header: [first, second, fragment],
                data: piece,
            } => {
                let kind = fragment & 0x3f;
                if fragment & 0x80 != 0 {
                    if open.is_some() {
                        return None;
                    }
                    data.extend_from_slice(&ANNEX_B_START_CODE);
                    data.extend_from_slice(&[(first & 0x81) | (kind << 1), *second]);
                    key |= is_irap(kind);
                    open = Some(kind);
                } else if open != Some(kind) {
                    return None;
                }
                data.extend_from_slice(piece);
                if fragment & 0x40 != 0 {
                    open = None;
                }
            }
        }
    }
    (open.is_none() && !data.is_empty()).then_some((data, key))
}
