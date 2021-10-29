var n1 = !true;  // !t returns false
var n2 = !false; // !f returns true
var n3 = !'Cat'; // !t returns false

assert n1 === false;
assert n2 === true;
assert n3 === false;
