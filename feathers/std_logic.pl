fn bool and(bool a, bool b) {
    RUST[std_func::and](a, b)
}

fn bool not(bool a) {
    RUST[std_func::not](a)
}

fn bool xor(bool a, bool b) {
    and(or(a, b), not(and(a, b)))
}

fn bool or(bool a, bool b) {
    not(and(not(a), not(b)))
}