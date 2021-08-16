import 'basic/unit.t' as unit
import 'basic/void.t' as void

struct Foo {
  x: Bool = 0
  y: unstable Bool = 1
  z: void.Void = 2
  w: unstable void.Void = 3
  s: unit.Unit = 4
  t: unstable unit.Unit = 5
}

choice Bar {
  x: Bool = 0
  y: unstable Bool = 1
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
