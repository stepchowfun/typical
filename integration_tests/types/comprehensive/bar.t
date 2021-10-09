choice Bar {
    p_required: [Unit] = 0
    q_required: [F64] = 1
    r_required: [S64] = 2
    s_required: [[String]] = 3
    t_required: Bool = 4
    u_required: Bytes = 5
    v_required: F64 = 6
    w_required: S64 = 7
    x_required: String = 8
    y_required: U64 = 9
    z_required = 10

    asymmetric p_asymmetric: [Unit] = 11
    asymmetric q_asymmetric: [F64] = 12
    asymmetric r_asymmetric: [S64] = 13
    asymmetric s_asymmetric: [[String]] = 14
    asymmetric t_asymmetric: Bool = 15
    asymmetric u_asymmetric: Bytes = 16
    asymmetric v_asymmetric: F64 = 17
    asymmetric w_asymmetric: S64 = 18
    asymmetric x_asymmetric: String = 19
    asymmetric y_asymmetric: U64 = 20
    asymmetric z_asymmetric = 21

    optional p_optional: [Unit] = 22
    optional q_optional: [F64] = 23
    optional r_optional: [S64] = 24
    optional s_optional: [[String]] = 25
    optional t_optional: Bool = 26
    optional u_optional: Bytes = 27
    optional v_optional: F64 = 28
    optional w_optional: S64 = 29
    optional x_optional: String = 30
    optional y_optional: U64 = 31
    optional z_optional = 32
}
