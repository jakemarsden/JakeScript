function multiply(a, b) {
  // TODO: Support `typeof` operator
  //b = typeof b !== 'undefined' ?  b : 1;
  b = b !== undefined ?  b : 1;

  return a * b;
}

multiply(5); // 5

console.assert(multiply(5) === 5);
