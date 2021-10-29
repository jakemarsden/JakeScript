function factorial(n) {
  // TODO: Braceless single-line if and loop statements
  if ((n === 0) || (n === 1)) {
    return 1;
  } else {
    return (n * factorial(n - 1));
  }
}

// TODO: Declare multiple variables in one statement
var a;
var b;
var c;
var d;
var e;
a = factorial(1); // a gets the value 1
b = factorial(2); // b gets the value 2
c = factorial(3); // c gets the value 6
d = factorial(4); // d gets the value 24
e = factorial(5); // e gets the value 120
assert a === 1;
assert b === 2;
assert c === 6;
assert d === 24;
assert e === 120;
