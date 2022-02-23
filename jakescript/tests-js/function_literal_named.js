let factorial = function fac(a) {
    if (a === 0 || a === 1) {
        return 1;
    }
    return a * fac(a - 1);
};
console.assert(factorial(5) === 120);
console.assert(factorial(10) === 3628800);

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
console.assert(fib(0) === 0);
console.assert(fib(1) === 1);
console.assert(fib(2) === 1);
console.assert(fib(3) === 2);
console.assert(fib(4) === 3);
console.assert(fib(5) === 5);
console.assert(fib(10) === 55);
console.assert(fib(12) === 144);

let shadowedByParam = function shadowed(shadowed, exp) {
    return shadowed ** exp;
};
console.assert(shadowedByParam(5, 3) === 125);

// TODO: Allow variables to be shadowed.
let shadowedByVar = function shadowed(n) {
    shadowed = n * n;
    return shadowed;
    var shadowed;
};
//console.assert(shadowedByVar(5) === 25);

let shadowedByInnerFunction = function shadowed(n) {
    return shadowed(n) ** 2;

    function shadowed(value) {
        return value + 1;
    }
};
//console.assert(shadowedByInnerFunction(3) === 16);
