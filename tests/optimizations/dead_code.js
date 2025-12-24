function dead_code() {
    let x = 5;
    return x; // Code after this line not compiled
    let y = 10;
    let z = 20;
    return y + z;
}

dead_code();