import 'basic/unit.t' as unit
import 'basic/void.t' as void

struct foo {
  x: bool = 0
  y: unstable bytes = 1
  z: unit.unit = 2
}

choice bar {
  x: bool = 0
  y: unstable bytes = 1
  z: void.void = 2
}
