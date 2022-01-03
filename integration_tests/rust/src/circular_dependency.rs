use {
    crate::{
        round_trip::check_match,
        types::circular_dependency::{
            dependency::types::{StructFromBelowIn, StructFromBelowOut},
            types::StructFromAboveOut,
        },
    },
    std::io,
};

pub fn run() -> io::Result<()> {
    check_match::<StructFromBelowOut, StructFromBelowIn>(StructFromBelowOut {
        x: StructFromAboveOut {},
    })?;

    Ok(())
}
