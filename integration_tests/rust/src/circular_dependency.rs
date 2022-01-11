use {
    crate::{
        assertions::assert_round_trip,
        types::circular_dependency::{
            dependency::types::{StructFromBelowIn, StructFromBelowOut},
            types::{StructFromAboveIn, StructFromAboveOut},
        },
    },
    std::io,
};

pub fn run() -> io::Result<()> {
    assert_round_trip::<StructFromAboveOut, StructFromAboveIn>(&StructFromAboveOut {
        field: "field".to_owned(),
        size: "size".to_owned(),
        elements: "elements".to_owned(),
        fallback: "fallback".to_owned(),
    })?;

    println!();

    assert_round_trip::<StructFromBelowOut, StructFromBelowIn>(&StructFromBelowOut {
        x: StructFromAboveOut {
            field: "field".to_owned(),
            size: "size".to_owned(),
            elements: "elements".to_owned(),
            fallback: "fallback".to_owned(),
        },
    })?;

    Ok(())
}
