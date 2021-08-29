import 'basic/unit.t' as unit
import 'basic/void.t' as void

struct Foo {
  x: Boolean = 0
  y: unstable Boolean = 1
  z: void.Void = 2
  w: unstable void.Void = 3
  s: unit.Unit = 4
  t: unstable unit.Unit = 5
}

choice Bar {
  x: Boolean = 0
  y: unstable Float64 = 1
  z: void.Void = 2
  w: unstable void.Void = 3
  s: unit.Unit = 4
  t: unstable unit.Unit = 5
}

struct FooAndBar {
  foo: Foo = 0
  bar: Bar = 1
}

choice FooOrBar {
  foo: Foo = 0
  bar: Bar = 1
}
