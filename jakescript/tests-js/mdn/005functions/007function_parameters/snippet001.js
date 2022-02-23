function multiply(a, b) {
  // TODO: Support `typeof` operator
  //b = typeof b !== 'undefined' ?  b : 1;
  b = b !== undefined ?  b : 1;

  return a * b;
}

console.assert(multiply(5) === 5, "5");
console.assert(multiply(5, 2) === 10);
