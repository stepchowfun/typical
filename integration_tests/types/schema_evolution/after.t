struct example_struct {
    required_to_required: string = 0
    asymmetric required_to_asymmetric: string = 1
    optional required_to_optional: string = 2
    # required_to_nonexistent: string = 3

    asymmetric_to_required: string = 4
    asymmetric asymmetric_to_asymmetric: string = 5
    optional asymmetric_to_optional: string = 6
    # asymmetric_to_nonexistent: string = 7

    # optional_none_to_required: string = 8 # This case would be an error.
    asymmetric optional_none_to_asymmetric: string = 9
    optional optional_none_to_optional: string = 10
    # optional_none_to_nonexistent: string = 11

    optional_some_to_required: string = 12
    asymmetric optional_some_to_asymmetric: string = 13
    optional optional_some_to_optional: string = 14
    # optional_some_to_nonexistent: string = 15

    # nonexistent_to_required: string = 16 # This case would be an error.
    asymmetric nonexistent_to_asymmetric: string = 17
    optional nonexistent_to_optional: string = 18
    # nonexistent_to_nonexistent: string = 19
}

choice example_choice {
    required_to_required: string = 0
    asymmetric required_to_asymmetric: string = 1
    # optional required_to_optional_handled: string = 2 # This case would be an error.
    # optional required_to_optional_fallback: string = 3 # This case would be an error.
    # required_to_nonexistent: string = 4 # This case would be an error.

    asymmetric_to_required: string = 5
    asymmetric asymmetric_to_asymmetric: string = 6
    optional asymmetric_to_optional_handled: string = 7
    optional asymmetric_to_optional_fallback: string = 8
    # asymmetric_to_nonexistent: string = 9

    optional_to_required: string = 10
    asymmetric optional_to_asymmetric: string = 11
    optional optional_to_optional_handled: string = 12
    optional optional_to_optional_fallback: string = 13
    # optional_to_nonexistent: string = 14

    nonexistent_to_required: string = 15
    asymmetric nonexistent_to_asymmetric: string = 16
    optional nonexistent_to_optional_handled: string = 17
    optional nonexistent_to_optional_fallback: string = 18
    # nonexistent_to_nonexistent: string = 19
}
