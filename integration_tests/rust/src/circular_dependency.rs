use {
    crate::{
        round_trip::check_match,
        types::circular_dependency::{
            dependency::types::{StructFromBelowIn, StructFromBelowOut},
            types::{StructFromAboveIn, StructFromAboveOut},
        },
    },
    std::io,
};

pub fn run() -> io::Result<()> {
    check_match::<StructFromAboveOut, StructFromAboveIn>(StructFromAboveOut {})?;

    check_match::<StructFromBelowOut, StructFromBelowIn>(StructFromBelowOut {
        x: StructFromAboveOut {},
    })?;

    Ok(())
}
