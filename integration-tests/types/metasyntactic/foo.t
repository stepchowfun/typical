struct foo {
  p_required: [unit] = 0
  q_required: [f64] = 1
  r_required: [s64] = 2
  s_required: [[string]] = 3
  t_required: bool = 4
  u_required: bytes = 5
  v_required: f64 = 6
  w_required: s64 = 7
  x_required: string = 8
  y_required: u64 = 9
  z_required: unit = 10

  unstable p_unstable: [unit] = 11
  unstable q_unstable: [f64] = 12
  unstable r_unstable: [s64] = 13
  unstable s_unstable: [[string]] = 14
  unstable t_unstable: bool = 15
  unstable u_unstable: bytes = 16
  unstable v_unstable: f64 = 17
  unstable w_unstable: s64 = 18
  unstable x_unstable: string = 19
  unstable y_unstable: u64 = 20
  unstable z_unstable: unit = 21

  optional p_optional: [unit] = 22
  optional q_optional: [f64] = 23
  optional r_optional: [s64] = 24
  optional s_optional: [[string]] = 25
  optional t_optional: bool = 26
  optional u_optional: bytes = 27
  optional v_optional: f64 = 28
  optional w_optional: s64 = 29
  optional x_optional: string = 30
  optional y_optional: u64 = 31
  optional z_optional: unit = 32
}
