function multiply(a, b) {
  // TODO: Support `typeof` operator
  //b = typeof b !== 'undefined' ?  b : 1;
  b = b !== undefined ?  b : 1;

  return a * b;
}

// FIXME: Function parameters which aren't passed should default to `undefined`
//console.assert(multiply(5) === 5, "5");
console.assert(multiply(5, undefined) === 5, "5");
