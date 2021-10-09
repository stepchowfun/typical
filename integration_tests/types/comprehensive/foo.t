struct Foo {
    p_required: [unit] = 0
    q_required: [f64] = 1
    r_required: [s64] = 2
    s_required: [[string]] = 3
    t_required: bool = 4
    u_required: bytes = 5
    v_required: f64 = 6
    w_required: s64 = 7
    x_required: string = 8
    y_required: u64 = 9
    z_required = 10

    asymmetric p_asymmetric: [unit] = 11
    asymmetric q_asymmetric: [f64] = 12
    asymmetric r_asymmetric: [s64] = 13
    asymmetric s_asymmetric: [[string]] = 14
    asymmetric t_asymmetric: bool = 15
    asymmetric u_asymmetric: bytes = 16
    asymmetric v_asymmetric: f64 = 17
    asymmetric w_asymmetric: s64 = 18
    asymmetric x_asymmetric: string = 19
    asymmetric y_asymmetric: u64 = 20
    asymmetric z_asymmetric = 21

    optional p_optional: [unit] = 22
    optional q_optional: [f64] = 23
    optional r_optional: [s64] = 24
    optional s_optional: [[string]] = 25
    optional t_optional: bool = 26
    optional u_optional: bytes = 27
    optional v_optional: f64 = 28
    optional w_optional: s64 = 29
    optional x_optional: string = 30
    optional y_optional: u64 = 31
    optional z_optional = 32
}
