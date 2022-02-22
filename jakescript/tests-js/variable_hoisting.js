console.assert(a === undefined);
a = 3;
console.assert(a === 3);
var a;
console.assert(a === 3);

// Variable declaration itself is hoisted, so the variable exists, but its initialiser is not
console.assert(b === undefined);
var b = 3 + 4;
console.assert(b === 7);
