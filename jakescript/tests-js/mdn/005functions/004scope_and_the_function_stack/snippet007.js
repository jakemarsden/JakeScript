function outside(x) {
  function inside(y) {
    return x + y;
  }
  return inside;
}
var fn_inside = outside(3); // Think of it like: give me a function that adds 3 to whatever you give
                        // it
var result = fn_inside(5); // returns 8
assert result === 8;

var result1 = outside(3)(5); // returns 8
assert result1 === 8;
