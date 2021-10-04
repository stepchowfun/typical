import 'metasyntactic/foo.t'
import 'metasyntactic/bar.t'

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
