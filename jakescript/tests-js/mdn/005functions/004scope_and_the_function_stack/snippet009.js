function outside() {
  var x = 5;
  function inside(x) {
    return x * 2;
  }
  return inside;
}

var result = outside()(10); // returns 20 instead of 10
assert result === 20;
