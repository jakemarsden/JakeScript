let a = 1000;
let b;
const c = 50;
var d = 10;
a = 13;
b = 100;
d = 4;
console.assert(a + b + c + d === 167);

let foo, bar;
console.assert(foo === undefined && bar === undefined);

let baz = 1, qux = 2;
console.assert(baz === 1 && qux === 2);
