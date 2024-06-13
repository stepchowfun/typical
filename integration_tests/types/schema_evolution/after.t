struct ExampleStruct {
    required_to_required: String = 0
    asymmetric required_to_asymmetric: String = 1
    optional required_to_optional: String = 2
    asymmetric_to_required: String = 4
    asymmetric asymmetric_to_asymmetric: String = 5
    optional asymmetric_to_optional: String = 6
    optional_to_required: String = 8
    asymmetric optional_to_asymmetric: String = 9
    optional optional_to_optional: String = 10
    asymmetric nonexistent_to_asymmetric = 13
    optional nonexistent_to_optional = 14

    deleted 3 7 11 12 15 16
}

choice ExampleChoice {
    required_to_required: String = 0
    asymmetric required_to_asymmetric: String = 1
    asymmetric_to_required: String = 4
    asymmetric asymmetric_to_asymmetric: String = 5
    optional asymmetric_to_optional: String = 6
    optional_to_required: String = 8
    asymmetric optional_to_asymmetric: String = 9
    optional optional_to_optional: String = 10
    nonexistent_to_required = 12
    asymmetric nonexistent_to_asymmetric = 13
    optional nonexistent_to_optional = 14

    deleted 2 3 7 11 15 16
}
