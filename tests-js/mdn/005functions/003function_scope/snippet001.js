// The following variables are defined in the global scope
// TODO: Declare multiple variables in one statement
var num1 = 20;
var num2 = 3;
var name = 'Chamakh';

// This function is defined in the global scope
function multiply() {
  return num1 * num2;
}

var result1 = multiply(); // Returns 60
assert result1 === 60;

// A nested function example
function getScore() {
  // TODO: Declare multiple variables in one statement
  // FIXME: Should not count as a duplicate declaration
  var num1_ = 2;
  var num2_ = 3;

  function add() {
    return name + ' scored ' + (num1_ + num2_);
  }

  return add();
}

var result2 = getScore(); // Returns "Chamakh scored 5"
assert result2 === "Chamakh scored 5";
