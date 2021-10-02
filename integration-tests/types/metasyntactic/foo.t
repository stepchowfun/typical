struct foo {
  t_required: bool = 0
  u_required: bytes = 1
  v_required: f64 = 2
  w_required: s64 = 3
  x_required: string = 4
  y_required: u64 = 5
  z_required: unit = 6

  unstable t_unstable: bool = 7
  unstable u_unstable: bytes = 8
  unstable v_unstable: f64 = 9
  unstable w_unstable: s64 = 10
  unstable x_unstable: string = 11
  unstable y_unstable: u64 = 12
  unstable z_unstable: unit = 13

  optional t_optional: bool = 14
  optional u_optional: bytes = 15
  optional v_optional: f64 = 16
  optional w_optional: s64 = 17
  optional x_optional: string = 18
  optional y_optional: u64 = 19
  optional z_optional: unit = 20
}
