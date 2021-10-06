struct example_struct {
  required_to_required: string = 0
  required_to_unstable: string = 1
  required_to_optional: string = 2
  required_to_nonexistent: string = 3

  unstable unstable_to_required: string = 4
  unstable unstable_to_unstable: string = 5
  unstable unstable_to_optional: string = 6
  unstable unstable_to_nonexistent: string = 7

  # optional optional_none_to_required: string = 8 # This case would be an error.
  optional optional_none_to_unstable: string = 9
  optional optional_none_to_optional: string = 10
  optional optional_none_to_nonexistent: string = 11

  optional optional_some_to_required: string = 12
  optional optional_some_to_unstable: string = 13
  optional optional_some_to_optional: string = 14
  optional optional_some_to_nonexistent: string = 15

  # nonexistent_to_required: string = 16 # This case would be an error.
  # nonexistent_to_unstable: string = 17
  # nonexistent_to_optional: string = 18
  # nonexistent_to_nonexistent: string = 19
}

choice example_choice {
  required_to_required: string = 0
  required_to_unstable: string = 1
  # required_to_optional_handled: string = 2 # This case would be an error.
  # required_to_optional_fallback: string = 3 # This case would be an error.
  # required_to_nonexistent: string = 4 # This case would be an error.

  unstable unstable_to_required: string = 5
  unstable unstable_to_unstable: string = 6
  unstable unstable_to_optional_handled: string = 7
  unstable unstable_to_optional_fallback: string = 8
  unstable unstable_to_nonexistent: string = 9

  optional optional_to_required: string = 10
  optional optional_to_unstable: string = 11
  optional optional_to_optional_handled: string = 12
  optional optional_to_optional_fallback: string = 13
  optional optional_to_nonexistent: string = 14

  # nonexistent_to_required: string = 15
  # nonexistent_to_unstable: string = 16
  # nonexistent_to_optional_handled: string = 17
  # nonexistent_to_optional_fallback: string = 18
  # nonexistent_to_nonexistent: string = 19
}
