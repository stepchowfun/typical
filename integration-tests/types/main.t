import 'basic/unit.t' as unit
import 'basic/void.t' as void

struct foo {
  v_required: bool = 0
  w_required: bytes = 1
  x_required: f64 = 2
  y_required: u64 = 3
  z_required: unit.unit = 4

  v_unstable: unstable bool = 5
  w_unstable: unstable bytes = 6
  x_unstable: unstable f64 = 7
  y_unstable: unstable u64 = 8
  z_unstable: unstable unit.unit = 9

  v_optional: optional bool = 10
  w_optional: optional bytes = 11
  x_optional: optional f64 = 12
  y_optional: optional u64 = 13
  z_optional: optional unit.unit = 14
}

choice bar {
  v_required: bool = 0
  w_required: bytes = 1
  x_required: f64 = 2
  y_required: u64 = 3
  z_required: unit.unit = 4

  v_unstable: unstable bool = 5
  w_unstable: unstable bytes = 6
  x_unstable: unstable f64 = 7
  y_unstable: unstable u64 = 8
  z_unstable: unstable unit.unit = 9

  v_optional: optional bool = 10
  w_optional: optional bytes = 11
  x_optional: optional f64 = 12
  y_optional: optional u64 = 13
  z_optional: optional unit.unit = 14
}
