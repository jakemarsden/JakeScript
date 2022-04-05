let factorial = function fac(a) {
    if (a === 0 || a === 1) {
        return 1;
    }
    return a * fac(a - 1);
};
console.assertEqual(factorial(5), 120);
console.assertEqual(factorial(10), 3628800);

let fib = function fib(n) {
    function add(a, b) {
        return a + b;
    }

    if (n === 0) {
        return 0;
    }
    if (n === 1) {
        return 1;
    }
    const a = fib(n - 2);
    let b = fib(n - 1);
    return add(a, b);
};
console.assertEqual(fib(0), 0);
console.assertEqual(fib(1), 1);
console.assertEqual(fib(2), 1);
console.assertEqual(fib(3), 2);
console.assertEqual(fib(4), 3);
console.assertEqual(fib(5), 5);
console.assertEqual(fib(10), 55);
console.assertEqual(fib(12), 144);

let shadowedByParam = function shadowed(shadowed, exp) {
    return shadowed ** exp;
};
console.assertEqual(shadowedByParam(5, 3), 125);

let shadowedByVar = function shadowed(n) {
    shadowed = n * n;
    return shadowed;
    var shadowed;
};
// TODO: Allow variables to be shadowed.
//console.assertEqual(shadowedByVar(5), 25);

let shadowedByInnerFunction = function shadowed(n) {
    return shadowed(n) ** 2;

    function shadowed(value) {
        return value + 1;
    }
};
// TODO: Allow variables to be shadowed.
//console.assertEqual(shadowedByInnerFunction(3), 16);
