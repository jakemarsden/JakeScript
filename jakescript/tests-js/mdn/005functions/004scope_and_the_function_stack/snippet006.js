function addSquares(a, b) {
  function square(x) {
    return x * x;
  }
  return square(a) + square(b);
}
console.assert(addSquares(2, 3) === 13, "returns 13");
console.assert(addSquares(3, 4) === 25, "returns 25");
console.assert(addSquares(4, 5) === 41, "returns 41");
