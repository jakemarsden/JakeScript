function outside() {
  var x = 5;
  function inside(x) {
    return x * 2;
  }
  return inside;
}

console.assert(outside()(10) === 20, "returns 20 instead of 10");
