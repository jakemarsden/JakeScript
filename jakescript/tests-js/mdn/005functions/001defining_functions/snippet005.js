const factorial = function fac(n) { return n < 2 ? 1 : n * fac(n - 1); };

assert factorial(3) === 6;
