console.assertEqual(evenFibonacciNumbers(89), 44);
console.assertEqual(evenFibonacciNumbers(4000000), 4613732);

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
