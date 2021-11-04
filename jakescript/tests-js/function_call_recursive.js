function factorial(a) {
    if (a === 0 || a === 1) {
        return 1;
    }
    return a * factorial(a - 1);
}
assert factorial(5) === 120;
assert factorial(10) === 3628800;

function fib(n) {
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
}
assert fib(0) === 0;
assert fib(1) === 1;
assert fib(2) === 1;
assert fib(3) === 2;
assert fib(4) === 3;
assert fib(5) === 5;
assert fib(10) === 55;
assert fib(12) === 144;

function isEvenByMutualRecursion(n) {
    if (n === 0) {
        return true;
    } else {
        return isOddByMutualRecursion(n - 1);
    }
}
function isOddByMutualRecursion(n) {
    if (n === 0) {
        return false;
    } else {
        return isEvenByMutualRecursion(n - 1);
    }
}
assert isEvenByMutualRecursion(3) === false;
assert isEvenByMutualRecursion(4);
assert isOddByMutualRecursion(5);
assert isOddByMutualRecursion(6) === false;
