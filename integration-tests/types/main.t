import 'metasyntactic/foo.t' as foo
import 'metasyntactic/bar.t' as bar

struct empty_struct {
}

choice empty_choice {
}

struct foo_and_bar {
  x: foo.foo = 0
  y: bar.bar = 1
}

choice foo_or_bar {
  x: foo.foo = 0
  y: bar.bar = 1
}
