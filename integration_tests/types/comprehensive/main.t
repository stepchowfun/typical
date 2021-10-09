import 'bar.t'
import 'foo.t'

struct EmptyStruct {
}

choice EmptyChoice {
}

struct FooAndBar {
    x: foo.Foo = 0
    y: bar.Bar = 1
}

choice FooOrBar {
    x: foo.Foo = 0
    y: bar.Bar = 1
}
