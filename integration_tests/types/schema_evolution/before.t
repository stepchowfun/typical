struct ExampleStruct {
    required_to_required: String = 0
    required_to_asymmetric: String = 1
    required_to_optional: String = 2
    required_to_nonexistent: String = 3

    asymmetric asymmetric_to_required: String = 4
    asymmetric asymmetric_to_asymmetric: String = 5
    asymmetric asymmetric_to_optional: String = 6
    asymmetric asymmetric_to_nonexistent: String = 7

    # optional optional_none_to_required: String = 8 # This case would be an error.
    optional optional_none_to_asymmetric: String = 9
    optional optional_none_to_optional: String = 10
    optional optional_none_to_nonexistent: String = 11

    optional optional_some_to_required: String = 12
    optional optional_some_to_asymmetric: String = 13
    optional optional_some_to_optional: String = 14
    optional optional_some_to_nonexistent: String = 15

    # nonexistent_to_required: String = 16 # This case would be an error.
    # nonexistent_to_asymmetric: String = 17
    # nonexistent_to_optional: String = 18
    # nonexistent_to_nonexistent: String = 19
}

choice ExampleChoice {
    required_to_required: String = 0
    required_to_asymmetric: String = 1
    # required_to_optional_handled: String = 2 # This case would be an error.
    # required_to_optional_fallback: String = 3 # This case would be an error.
    # required_to_nonexistent: String = 4 # This case would be an error.

    asymmetric asymmetric_to_required: String = 5
    asymmetric asymmetric_to_asymmetric: String = 6
    asymmetric asymmetric_to_optional_handled: String = 7
    asymmetric asymmetric_to_optional_fallback: String = 8
    asymmetric asymmetric_to_nonexistent: String = 9

    optional optional_to_required: String = 10
    optional optional_to_asymmetric: String = 11
    optional optional_to_optional_handled: String = 12
    optional optional_to_optional_fallback: String = 13
    optional optional_to_nonexistent: String = 14

    # nonexistent_to_required: String = 15
    # nonexistent_to_asymmetric: String = 16
    # nonexistent_to_optional_handled: String = 17
    # nonexistent_to_optional_fallback: String = 18
    # nonexistent_to_nonexistent: String = 19
}
