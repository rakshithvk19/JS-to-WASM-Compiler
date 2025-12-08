function logical() {
    let a = 5 && 3;
    let b = 0 && 3;
    let c = 5 || 3;
    let d = 0 || 3;
    let e = 1 && 2 && 3;
    let f = 0 || 0 || 7;
    return a + b + c + d + e + f;
}

logical();