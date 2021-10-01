import 'basic/unit.t' as unit
import 'basic/void.t' as void

struct foo {
  u_required: bool = 0
  v_required: bytes = 1
  w_required: f64 = 2
  x_required: string = 3
  y_required: u64 = 4
  z_required: unit.unit = 5

  u_unstable: unstable bool = 6
  v_unstable: unstable bytes = 7
  w_unstable: unstable f64 = 8
  x_unstable: unstable string = 9
  y_unstable: unstable u64 = 10
  z_unstable: unstable unit.unit = 11

  u_optional: optional bool = 12
  v_optional: optional bytes = 13
  w_optional: optional f64 = 14
  x_optional: optional string = 15
  y_optional: optional u64 = 16
  z_optional: optional unit.unit = 17
}

choice bar {
  u_required: bool = 0
  v_required: bytes = 1
  w_required: f64 = 2
  x_required: string = 3
  y_required: u64 = 4
  z_required: unit.unit = 5

  u_unstable: unstable bool = 6
  v_unstable: unstable bytes = 7
  w_unstable: unstable f64 = 8
  x_unstable: unstable string = 9
  y_unstable: unstable u64 = 10
  z_unstable: unstable unit.unit = 11

  u_optional: optional bool = 12
  v_optional: optional bytes = 13
  w_optional: optional f64 = 14
  x_optional: optional string = 15
  y_optional: optional u64 = 16
  z_optional: optional unit.unit = 17
}
