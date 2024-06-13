struct ExampleStruct {
    required_to_required: String = 0
    required_to_asymmetric: String = 1
    required_to_optional: String = 2
    required_to_nonexistent: String = 3
    asymmetric asymmetric_to_required: String = 4
    asymmetric asymmetric_to_asymmetric: String = 5
    asymmetric asymmetric_to_optional: String = 6
    asymmetric asymmetric_to_nonexistent: String = 7
    optional optional_to_required: String = 8
    optional optional_to_asymmetric: String = 9
    optional optional_to_optional: String = 10
    optional optional_to_nonexistent: String = 11

    deleted 12 13 14 15
}

choice ExampleChoice {
    required_to_required: String = 0
    required_to_asymmetric: String = 1
    asymmetric asymmetric_to_required: String = 4
    asymmetric asymmetric_to_asymmetric: String = 5
    asymmetric asymmetric_to_optional: String = 6
    asymmetric asymmetric_to_nonexistent: String = 7
    optional optional_to_required: String = 8
    optional optional_to_asymmetric: String = 9
    optional optional_to_optional: String = 10
    optional optional_to_nonexistent: String = 11

    deleted 2 3 12 13 14 15
}
