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
    assert_round_trip::<StructFromAboveOut, StructFromAboveIn>(&StructFromAboveOut {})?;

    assert_round_trip::<StructFromBelowOut, StructFromBelowIn>(&StructFromBelowOut {
        x: StructFromAboveOut {},
    })?;

    Ok(())
}
