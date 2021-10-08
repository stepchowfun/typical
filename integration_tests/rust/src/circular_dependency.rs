use {
    crate::{
        round_trip::check_match,
        types::circular_dependency::{
            dependency::main::{StructFromBelowIn, StructFromBelowOut},
            main::StructFromAboveOut,
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
