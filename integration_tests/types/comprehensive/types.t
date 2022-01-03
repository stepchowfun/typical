import 'bar.t'
import 'foo.t'

struct FooAndBar {
    x: foo.Foo = 0
    y: bar.Bar = 1
}

choice FooOrBar {
    x: foo.Foo = 0
    y: bar.Bar = 1
}
