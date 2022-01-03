use {
    crate::{
        round_trip::check_ok,
        types::schema_evolution::{
            after, before,
            types::{SingletonChoiceIn, SingletonChoiceOut, SingletonStructIn, SingletonStructOut},
        },
    },
    std::io,
};

pub fn run() -> io::Result<()> {
    check_ok::<before::ExampleStructOut, after::ExampleStructIn>(&before::ExampleStructOut {
        required_to_required: "required_to_required".to_owned(),
        required_to_asymmetric: "required_to_asymmetric".to_owned(),
        required_to_optional: "required_to_optional".to_owned(),
        required_to_nonexistent: "required_to_nonexistent".to_owned(),
        asymmetric_to_required: "asymmetric_to_required".to_owned(),
        asymmetric_to_asymmetric: "asymmetric_to_asymmetric".to_owned(),
        asymmetric_to_optional: "asymmetric_to_optional".to_owned(),
        asymmetric_to_nonexistent: "asymmetric_to_nonexistent".to_owned(),
        optional_none_to_asymmetric: None,
        optional_none_to_optional: None,
        optional_none_to_nonexistent: None,
        optional_some_to_required: Some("optional_some_to_required".to_owned()),
        optional_some_to_asymmetric: Some("optional_some_to_asymmetric".to_owned()),
        optional_some_to_optional: Some("optional_some_to_optional".to_owned()),
        optional_some_to_nonexistent: Some("optional_some_to_nonexistent".to_owned()),
    })?;

    let fallback = before::ExampleChoiceOut::RequiredToRequired("RequiredToRequired".to_owned());

    check_ok::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::RequiredToRequired("RequiredToRequired".to_owned()),
    )?;

    check_ok::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::RequiredToAsymmetric("RequiredToAsymmetric".to_owned()),
    )?;

    check_ok::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::AsymmetricToRequired(
            "AsymmetricToRequired".to_owned(),
            Box::new(fallback.clone()),
        ),
    )?;

    check_ok::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::AsymmetricToAsymmetric(
            "AsymmetricToAsymmetric".to_owned(),
            Box::new(fallback.clone()),
        ),
    )?;

    check_ok::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::AsymmetricToOptionalHandled(
            "AsymmetricToOptionalHandled".to_owned(),
            Box::new(fallback.clone()),
        ),
    )?;

    check_ok::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::AsymmetricToOptionalFallback(
            "AsymmetricToOptionalFallback".to_owned(),
            Box::new(fallback.clone()),
        ),
    )?;

    check_ok::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::AsymmetricToNonexistent(
            "AsymmetricToNonexistent".to_owned(),
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
        &before::ExampleChoiceOut::OptionalToAsymmetric(
            "OptionalToAsymmetric".to_owned(),
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

    check_ok::<SingletonStructOut, SingletonChoiceIn>(&SingletonStructOut {
        x: "foo".to_owned(),
    })?;

    check_ok::<SingletonChoiceOut, SingletonStructIn>(&SingletonChoiceOut::X("foo".to_owned()))?;

    Ok(())
}
