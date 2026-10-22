use crate::{OFFER_TABLE, ProtoField};

pub(crate) fn table_fields() -> Vec<ProtoField> {
    OFFER_TABLE
        .iter()
        .map(|row| {
            let cells: Vec<ProtoField> = (1..)
                .zip(row.iter())
                .map(|(number, value)| ProtoField::varint(number, *value))
                .collect();
            ProtoField::message(9, &cells)
        })
        .collect()
}
