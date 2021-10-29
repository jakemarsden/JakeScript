function addSquares(a, b) {
  function square(x) {
    return x * x;
  }
  return square(a) + square(b);
}
var a = addSquares(2, 3); // returns 13
var b = addSquares(3, 4); // returns 25
var c = addSquares(4, 5); // returns 41
assert a === 13;
assert b === 25;
assert c === 41;
