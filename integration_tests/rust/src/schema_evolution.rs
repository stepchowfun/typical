use crate::{
    round_trip::check_ok,
    types::schema_evolution::{after, before},
};
use std::io;

pub fn run() -> io::Result<()> {
    check_ok::<before::ExampleStructOut, after::ExampleStructIn>(&before::ExampleStructOut {
        required_to_required: "required_to_required".to_owned(),
        required_to_unstable: "required_to_unstable".to_owned(),
        required_to_optional: "required_to_optional".to_owned(),
        required_to_nonexistent: "required_to_nonexistent".to_owned(),
        unstable_to_required: "unstable_to_required".to_owned(),
        unstable_to_unstable: "unstable_to_unstable".to_owned(),
        unstable_to_optional: "unstable_to_optional".to_owned(),
        unstable_to_nonexistent: "unstable_to_nonexistent".to_owned(),
        optional_none_to_unstable: None,
        optional_none_to_optional: None,
        optional_none_to_nonexistent: None,
        optional_some_to_required: Some("optional_some_to_required".to_owned()),
        optional_some_to_unstable: Some("optional_some_to_unstable".to_owned()),
        optional_some_to_optional: Some("optional_some_to_optional".to_owned()),
        optional_some_to_nonexistent: Some("optional_some_to_nonexistent".to_owned()),
    })?;

    let fallback = before::ExampleChoiceOut::RequiredToRequired("RequiredToRequired".to_owned());

    check_ok::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::RequiredToRequired("RequiredToRequired".to_owned()),
    )?;

    check_ok::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::RequiredToUnstable("RequiredToUnstable".to_owned()),
    )?;

    check_ok::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::UnstableToRequired(
            "UnstableToRequired".to_owned(),
            Box::new(fallback.clone()),
        ),
    )?;

    check_ok::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::UnstableToUnstable(
            "UnstableToUnstable".to_owned(),
            Box::new(fallback.clone()),
        ),
    )?;

    check_ok::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::UnstableToOptionalHandled(
            "UnstableToOptionalHandled".to_owned(),
            Box::new(fallback.clone()),
        ),
    )?;

    check_ok::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::UnstableToOptionalFallback(
            "UnstableToOptionalFallback".to_owned(),
            Box::new(fallback.clone()),
        ),
    )?;

    check_ok::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::UnstableToNonexistent(
            "UnstableToNonexistent".to_owned(),
            Box::new(fallback.clone()),
        ),
    )?;

    check_ok::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::OptionalToRequired(
            "OptionalToRequired".to_owned(),
            Box::new(fallback.clone()),
        ),
    )?;

    check_ok::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::OptionalToUnstable(
            "OptionalToUnstable".to_owned(),
            Box::new(fallback.clone()),
        ),
    )?;

    check_ok::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::OptionalToOptionalHandled(
            "OptionalToOptionalHandled".to_owned(),
            Box::new(fallback.clone()),
        ),
    )?;

    check_ok::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::OptionalToOptionalFallback(
            "OptionalToOptionalFallback".to_owned(),
            Box::new(fallback.clone()),
        ),
    )?;

    check_ok::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::OptionalToNonexistent(
            "OptionalToNonexistent".to_owned(),
            Box::new(fallback),
        ),
    )?;

    Ok(())
}
