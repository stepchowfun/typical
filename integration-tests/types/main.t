import 'basic/unit.t' as unit
import 'basic/void.t' as void

struct foo {
  t_required: bool = 0
  u_required: bytes = 1
  v_required: f64 = 2
  w_required: s64 = 3
  x_required: string = 4
  y_required: u64 = 5
  z_required: unit.unit = 6

  t_unstable: unstable bool = 7
  u_unstable: unstable bytes = 8
  v_unstable: unstable f64 = 9
  w_unstable: unstable s64 = 10
  x_unstable: unstable string = 11
  y_unstable: unstable u64 = 12
  z_unstable: unstable unit.unit = 13

  t_optional: optional bool = 14
  u_optional: optional bytes = 15
  v_optional: optional f64 = 16
  w_optional: optional s64 = 17
  x_optional: optional string = 18
  y_optional: optional u64 = 19
  z_optional: optional unit.unit = 20
}

choice bar {
  t_required: bool = 0
  u_required: bytes = 1
  v_required: f64 = 2
  w_required: s64 = 3
  x_required: string = 4
  y_required: u64 = 5
  z_required: unit.unit = 6

  t_unstable: unstable bool = 7
  u_unstable: unstable bytes = 8
  v_unstable: unstable f64 = 9
  w_unstable: unstable s64 = 10
  x_unstable: unstable string = 11
  y_unstable: unstable u64 = 12
  z_unstable: unstable unit.unit = 13

  t_optional: optional bool = 14
  u_optional: optional bytes = 15
  v_optional: optional f64 = 16
  w_optional: optional s64 = 17
  x_optional: optional string = 18
  y_optional: optional u64 = 19
  z_optional: optional unit.unit = 20
}
