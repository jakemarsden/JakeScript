var a1 =  true && true;     // t && t returns true
var a2 =  true && false;    // t && f returns false
var a3 = false && true;     // f && t returns false
var a4 = false && (3 == 4); // f && f returns false
var a5 = 'Cat' && 'Dog';    // t && t returns Dog
var a6 = false && 'Cat';    // f && t returns false
var a7 = 'Cat' && false;    // t && f returns false

console.assert(a1 === true);
console.assert(a2 === false);
console.assert(a3 === false);
console.assert(a4 === false);
console.assert(a5 === "Dog");
console.assert(a6 === false);
console.assert(a7 === false);
