use {
    crate::{
        round_trip::check_match,
        types::degenerate::types::{EmptyChoiceIn, EmptyChoiceOut, EmptyStructIn, EmptyStructOut},
    },
    std::io,
};

#[allow(clippy::needless_pass_by_value)]
fn _initial_in<T>(x: EmptyChoiceIn) -> T {
    match x {}
}

#[allow(clippy::needless_pass_by_value)]
fn _initial_out<T>(x: EmptyChoiceOut) -> T {
    match x {}
}

fn _terminal_in<T>(_: T) -> EmptyStructIn {
    EmptyStructIn {}
}

fn _terminal_out<T>(_: T) -> EmptyStructOut {
    EmptyStructOut {}
}

pub fn run() -> io::Result<()> {
    check_match::<EmptyStructOut, EmptyStructIn>(EmptyStructOut {})?;

    Ok(())
}
