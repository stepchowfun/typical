use {
    crate::{
        assertions::assert_match,
        types::schema_evolution::{
            after, before,
            types::{SingletonChoiceIn, SingletonChoiceOut, SingletonStructIn, SingletonStructOut},
        },
    },
    std::io,
};

#[allow(clippy::too_many_lines)]
pub fn run() -> io::Result<()> {
    assert_match::<before::ExampleStructOut, after::ExampleStructIn>(
        &before::ExampleStructOut {
            required_to_required: "required_to_required".to_owned(),
            required_to_asymmetric: "required_to_asymmetric".to_owned(),
            required_to_optional: "required_to_optional".to_owned(),
            required_to_nonexistent: "required_to_nonexistent".to_owned(),
            asymmetric_to_required: "asymmetric_to_required".to_owned(),
            asymmetric_to_asymmetric: "asymmetric_to_asymmetric".to_owned(),
            asymmetric_to_optional: "asymmetric_to_optional".to_owned(),
            asymmetric_to_nonexistent: "asymmetric_to_nonexistent".to_owned(),
            optional_to_required: Some("optional_to_required".to_owned()),
            optional_to_asymmetric: None,
            optional_to_optional: None,
            optional_to_nonexistent: None,
        },
        &after::ExampleStructIn {
            required_to_required: "required_to_required".to_owned(),
            required_to_asymmetric: Some("required_to_asymmetric".to_owned()),
            required_to_optional: Some("required_to_optional".to_owned()),
            asymmetric_to_required: "asymmetric_to_required".to_owned(),
            asymmetric_to_asymmetric: Some("asymmetric_to_asymmetric".to_owned()),
            asymmetric_to_optional: Some("asymmetric_to_optional".to_owned()),
            optional_to_required: "optional_to_required".to_owned(),
            optional_to_asymmetric: None,
            optional_to_optional: None,
            nonexistent_to_asymmetric: None,
            nonexistent_to_optional: None,
        },
    )?;

    assert_match::<before::ExampleStructOut, after::ExampleStructIn>(
        &before::ExampleStructOut {
            required_to_required: "required_to_required".to_owned(),
            required_to_asymmetric: "required_to_asymmetric".to_owned(),
            required_to_optional: "required_to_optional".to_owned(),
            required_to_nonexistent: "required_to_nonexistent".to_owned(),
            asymmetric_to_required: "asymmetric_to_required".to_owned(),
            asymmetric_to_asymmetric: "asymmetric_to_asymmetric".to_owned(),
            asymmetric_to_optional: "asymmetric_to_optional".to_owned(),
            asymmetric_to_nonexistent: "asymmetric_to_nonexistent".to_owned(),
            optional_to_required: Some("optional_to_required".to_owned()),
            optional_to_asymmetric: Some("optional_to_asymmetric".to_owned()),
            optional_to_optional: Some("optional_to_optional".to_owned()),
            optional_to_nonexistent: Some("optional_to_nonexistent".to_owned()),
        },
        &after::ExampleStructIn {
            required_to_required: "required_to_required".to_owned(),
            required_to_asymmetric: Some("required_to_asymmetric".to_owned()),
            required_to_optional: Some("required_to_optional".to_owned()),
            asymmetric_to_required: "asymmetric_to_required".to_owned(),
            asymmetric_to_asymmetric: Some("asymmetric_to_asymmetric".to_owned()),
            asymmetric_to_optional: Some("asymmetric_to_optional".to_owned()),
            optional_to_required: "optional_to_required".to_owned(),
            optional_to_asymmetric: Some("optional_to_asymmetric".to_owned()),
            optional_to_optional: Some("optional_to_optional".to_owned()),
            nonexistent_to_asymmetric: None,
            nonexistent_to_optional: None,
        },
    )?;

    println!();

    let fallback_before =
        before::ExampleChoiceOut::RequiredToRequired("required_to_required".to_owned());
    let fallback_after =
        after::ExampleChoiceIn::RequiredToRequired("required_to_required".to_owned());

    assert_match::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::RequiredToRequired("required_to_required".to_owned()),
        &after::ExampleChoiceIn::RequiredToRequired("required_to_required".to_owned()),
    )?;

    assert_match::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::RequiredToAsymmetric("required_to_asymmetric".to_owned()),
        &after::ExampleChoiceIn::RequiredToAsymmetric("required_to_asymmetric".to_owned()),
    )?;

    assert_match::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::AsymmetricToRequired(
            "asymmetric_to_required".to_owned(),
            Box::new(fallback_before.clone()),
        ),
        &after::ExampleChoiceIn::AsymmetricToRequired("asymmetric_to_required".to_owned()),
    )?;

    assert_match::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::AsymmetricToAsymmetric(
            "asymmetric_to_asymmetric".to_owned(),
            Box::new(fallback_before.clone()),
        ),
        &after::ExampleChoiceIn::AsymmetricToAsymmetric("asymmetric_to_asymmetric".to_owned()),
    )?;

    assert_match::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::AsymmetricToOptional(
            "asymmetric_to_optional".to_owned(),
            Box::new(fallback_before.clone()),
        ),
        &after::ExampleChoiceIn::AsymmetricToOptional(
            "asymmetric_to_optional".to_owned(),
            Box::new(fallback_after.clone()),
        ),
    )?;

    assert_match::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::AsymmetricToNonexistent(
            "asymmetric_to_nonexistent".to_owned(),
            Box::new(fallback_before.clone()),
        ),
        &fallback_after,
    )?;

    assert_match::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::OptionalToRequired(
            "optional_to_required".to_owned(),
            Box::new(fallback_before.clone()),
        ),
        &after::ExampleChoiceIn::OptionalToRequired("optional_to_required".to_owned()),
    )?;

    assert_match::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::OptionalToAsymmetric(
            "optional_to_asymmetric".to_owned(),
            Box::new(fallback_before.clone()),
        ),
        &after::ExampleChoiceIn::OptionalToAsymmetric("optional_to_asymmetric".to_owned()),
    )?;

    assert_match::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::OptionalToOptional(
            "optional_to_optional".to_owned(),
            Box::new(fallback_before.clone()),
        ),
        &after::ExampleChoiceIn::OptionalToOptional(
            "optional_to_optional".to_owned(),
            Box::new(fallback_after.clone()),
        ),
    )?;

    assert_match::<before::ExampleChoiceOut, after::ExampleChoiceIn>(
        &before::ExampleChoiceOut::OptionalToNonexistent(
            "optional_to_nonexistent".to_owned(),
            Box::new(fallback_before),
        ),
        &fallback_after,
    )?;

    assert_match::<SingletonStructOut, SingletonChoiceIn>(
        &SingletonStructOut {
            x: "foo".to_owned(),
        },
        &SingletonChoiceIn::X("foo".to_owned()),
    )?;

    assert_match::<SingletonChoiceOut, SingletonStructIn>(
        &SingletonChoiceOut::X("foo".to_owned()),
        &SingletonStructIn {
            x: "foo".to_owned(),
        },
    )?;

    Ok(())
}
