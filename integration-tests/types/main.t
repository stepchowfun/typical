import 'basic/unit.t' as unit
import 'basic/void.t' as void

struct foo {
  x: bool = 0
  y: unstable bool = 1
  z: void.void = 2
  w: unstable void.void = 3
  s: unit.unit = 4
  t: unstable unit.unit = 5
}

choice bar {
  x: bool = 0
  y: unstable f64 = 1
  z: void.void = 2
  w: unstable void.void = 3
  s: unit.unit = 4
  t: unstable unit.unit = 5
}

struct foo_and_bar {
  foo: foo = 0
  bar: bar = 1
}

choice foo_or_bar {
  foo: foo = 0
  bar: bar = 1
}

struct baz {
  x: bool = 0
  y: u64 = 1
  z: f64 = 2
}

choice qux {
  x: bool = 0
  y: bytes = 1
  z: unstable f64 = 2
}
