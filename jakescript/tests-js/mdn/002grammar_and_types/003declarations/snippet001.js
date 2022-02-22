var a;
console.assert('The value of a is ' + a === "The value of a is undefined");

console.assert('The value of b is ' + b === "The value of b is undefined");
var b;
// This one may puzzle you until you read 'Variable hoisting' below

//console.log('The value of c is ' + c); // Uncaught ReferenceError: c is not defined

let x;
console.assert('The value of x is ' + x === "The value of x is undefined");

//console.log('The value of y is ' + y); // Uncaught ReferenceError: y is not defined
let y;
