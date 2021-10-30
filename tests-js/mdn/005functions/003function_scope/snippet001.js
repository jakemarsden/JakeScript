// The following variables are defined in the global scope
var num1 = 20,
    num2 = 3,
    name = 'Chamakh';

// This function is defined in the global scope
function multiply() {
  return num1 * num2;
}

var result1 = multiply(); // Returns 60
assert result1 === 60;

// A nested function example
function getScore() {
  // FIXME: Should not count as a duplicate declaration
  var num1_ = 2,
      num2_ = 3;

  function add() {
    return name + ' scored ' + (num1_ + num2_);
  }

  return add();
}

var result2 = getScore(); // Returns "Chamakh scored 5"
assert result2 === "Chamakh scored 5";
