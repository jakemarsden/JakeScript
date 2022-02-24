var mockLog = "";
var parentLogger = console.log;
console.log = function(msg) {
  mockLog += msg + '\n';
  parentLogger(msg);
};

var a;
console.log('The value of a is ' + a); // The value of a is undefined

console.log('The value of b is ' + b); // The value of b is undefined
var b;
// This one may puzzle you until you read 'Variable hoisting' below

//console.log('The value of c is ' + c); // Uncaught ReferenceError: c is not defined

let x;
console.log('The value of x is ' + x); // The value of x is undefined

//console.log('The value of y is ' + y); // Uncaught ReferenceError: y is not defined
let y;

console.assert(mockLog === "The value of a is undefined\nThe value of b is undefined\nThe value of x is undefined\n");
