var mystring = 'alpha';
//mystring += 'bet'; // evaluates to "alphabet" and assigns this value to mystring.

console.assert((mystring += 'bet') === "alphabet");
console.assert(mystring === "alphabet");
