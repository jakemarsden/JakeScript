// TODO: Support function literals with names (so they can call themselves)
const factorial = function (n) { return n < 2 ? 1 : n * factorial(n - 1); };

assert factorial(3) === 6;
