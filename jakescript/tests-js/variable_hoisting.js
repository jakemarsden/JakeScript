assert a === undefined;
a = 3;
assert a === 3;
var a;
assert a === 3;

// Variable declaration itself is hoisted, so the variable exists, but its initialiser is not
assert b === undefined;
var b = 3 + 4;
assert b === 7;
