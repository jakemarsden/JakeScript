var log;

function A(x) {
  function B(y) {
    function C(z) {
      log = x + y + z;
    }
    C(3);
  }
  B(2);
}
A(1); // logs 6 (1 + 2 + 3)
assert log === 6;
