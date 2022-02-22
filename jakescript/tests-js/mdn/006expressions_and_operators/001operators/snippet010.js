var n1 = !true;  // !t returns false
var n2 = !false; // !f returns true
var n3 = !'Cat'; // !t returns false

console.assert(n1 === false);
console.assert(n2 === true);
console.assert(n3 === false);
