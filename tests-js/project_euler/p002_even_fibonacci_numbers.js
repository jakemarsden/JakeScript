function evenFibonacciNumbers(limit) {
    let f0 = 0;
    let f1 = 1;
    let sum = 0;
    while (f1 < limit) {
        if (f1 % 2 === 0) {
            sum += f1;
        }
        let f2 = f0 + f1;
        f0 = f1;
        f1 = f2;
    }
    return sum;
}

assert evenFibonacciNumbers(89) === 44;
assert evenFibonacciNumbers(4_000_000) === 4_613_732;
