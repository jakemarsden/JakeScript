var log;

var a;
log = 'The value of a is ' + a; // The value of a is undefined
assert log === "The value of a is undefined";

log = 'The value of b is ' + b; // The value of b is undefined
assert log === "The value of b is undefined";
var b;
// This one may puzzle you until you read 'Variable hoisting' below

let x;
log = 'The value of x is ' + x; // The value of x is undefined
assert log === "The value of x is undefined";
