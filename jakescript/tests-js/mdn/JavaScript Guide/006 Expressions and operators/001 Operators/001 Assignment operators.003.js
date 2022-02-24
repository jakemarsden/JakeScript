function f() { return "f"; }
function g() { return "g"; }

// Declares a variable x and initializes it to the result of f().
// The result of the x = f() assignment expression is discarded.
let x = f();

console.assert(x === "f");

// Declares a variable x and initializes it to the result of g().
// The result of the x = g() assignment expression is discarded.
x = g(); // Reassigns the variable x to the result of g().

console.assert(x === "g");
