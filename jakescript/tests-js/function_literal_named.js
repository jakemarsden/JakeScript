let factorial = function fac(a) {
    if (a === 0 || a === 1) {
        return 1;
    }
    return a * fac(a - 1);
};
assert factorial(5) === 120;
assert factorial(10) === 3628800;

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
assert fib(0) === 0;
assert fib(1) === 1;
assert fib(2) === 1;
assert fib(3) === 2;
assert fib(4) === 3;
assert fib(5) === 5;
assert fib(10) === 55;
assert fib(12) === 144;

let shadowedByParam = function shadowed(shadowed, exp) {
    return shadowed ** exp;
};
assert shadowedByParam(5, 3) === 125;

// TODO: Allow variables to be shadowed.
let shadowedByVar = function shadowed(n) {
    shadowed = n * n;
    return shadowed;
    var shadowed;
};
//assert shadowedByVar(5) === 25;

let shadowedByInnerFunction = function shadowed(n) {
    return shadowed(n) ** 2;

    function shadowed(value) {
        return value + 1;
    }
};
//assert shadowedByInnerFunction(3) === 16;
