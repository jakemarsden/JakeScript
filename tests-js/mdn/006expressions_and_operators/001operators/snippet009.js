var o1 =  true || true;     // t || t returns true
var o2 = false || true;     // f || t returns true
var o3 =  true || false;    // t || f returns true
var o4 = false || (3 == 4); // f || f returns false
var o5 = 'Cat' || 'Dog';    // t || t returns Cat
var o6 = false || 'Cat';    // f || t returns Cat
var o7 = 'Cat' || false;    // t || f returns Cat

assert o1 === true;
assert o2 === true;
assert o3 === true;
assert o4 === false;
// FIXME: assert o5 === "Cat";
// FIXME: assert o6 === "Cat";
// FIXME: assert o7 === "Cat";
