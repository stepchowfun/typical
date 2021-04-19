import 'basic/unit.t' as unit
import 'basic/void.t' as void

struct Foo {
  x: unit.Unit = 0
  y: void.Void = 1
  z: restricted unit.Unit = 2
  w: Bool = 3
}

choice Bar {
  x: unit.Unit = 0
  y: void.Void = 1
  z: restricted unit.Unit = 2
  w: Bool = 3
}
