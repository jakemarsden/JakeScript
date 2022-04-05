function factorial(a) {
    if (a === 0 || a === 1) {
        return 1;
    }
    return a * factorial(a - 1);
}
console.assertEqual(factorial(5), 120);
console.assertEqual(factorial(10), 3628800);

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
console.assertEqual(fib(0), 0);
console.assertEqual(fib(1), 1);
console.assertEqual(fib(2), 1);
console.assertEqual(fib(3), 2);
console.assertEqual(fib(4), 3);
console.assertEqual(fib(5), 5);
console.assertEqual(fib(10), 55);
console.assertEqual(fib(12), 144);

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
console.assert(!isEvenByMutualRecursion(3));
console.assert(isEvenByMutualRecursion(4));
console.assert(isOddByMutualRecursion(5));
console.assert(!isOddByMutualRecursion(6));
