# A trivial struct with no fields
struct Unit {
}

# The familiar Boolean type
choice Bool {
  false: Unit = 0
  true: Unit = 1
}
