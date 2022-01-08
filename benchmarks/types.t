struct Struct {
    x: String = 0
}

choice Choice {
    x: String = 0
}

struct Message {
    a = 0
    b: F64 = 1
    c: U64 = 2
    d: S64 = 3
    e: Bool = 4
    f: Bytes = 5
    g: String = 6
    h: Struct = 7
    i: Choice = 8
    j: [Unit] = 9
    k: [F64] = 10
    l: [U64] = 11
    m: [S64] = 12
    n: [Bool] = 13
    o: [Bytes] = 14
    p: [String] = 15
    q: [Struct] = 16
    r: [Choice] = 17
    s: [[Unit]] = 18
    t: [[F64]] = 19
    u: [[U64]] = 20
    v: [[S64]] = 21
    w: [[Bool]] = 22
    x: [[Bytes]] = 23
    y: [[String]] = 24
    z: [[Struct]] = 25
    aa: [[Choice]] = 26
}
