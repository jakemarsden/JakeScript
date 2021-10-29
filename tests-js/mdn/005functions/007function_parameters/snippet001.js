function multiply(a, b) {
  // TODO: Support `typeof` operator
  //b = typeof b !== 'undefined' ?  b : 1;
  b = b !== undefined ? b : 1;

  return a * b;
}

// FIXME: Function parameters which aren't passed should default to `undefined`
//var result = multiply(5); // 5
//assert result === 5;
