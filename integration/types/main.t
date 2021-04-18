import 'basic/bool.t' as bool

struct Foo {
  x: bool.Bool = 0
  y: bool.Bool = 1
  z: restricted bool.Bool = 2
}

choice Bar {
  x: bool.Bool = 0
  y: bool.Bool = 1
  z: restricted bool.Bool = 2
}
