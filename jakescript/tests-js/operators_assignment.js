let z;
z = 20;
console.assert(z === 20);

let a = 20;
a += 10;
console.assert(a === 30);
let b = 20;
b -= 10;
console.assert(b === 10);
let c = 20;
c *= 10;
console.assert(c === 200);
let d = 20;
d /= 10;
console.assert(d === 2);
let e = 20;
e %= 7;
console.assert(e === 6);
let f = 20;
f **= 3;
console.assert(f === 8000);

let zz;
console.assert((zz = 20) === 20);

let aa = 20;
console.assert((aa += 10) === 30);
let bb = 20;
console.assert((bb -= 10) === 10);
let cc = 20;
console.assert((cc *= 10) === 200);
let dd = 20;
console.assert((dd /= 10) === 2);
let ee = 20;
console.assert((ee %= 7) === 6);
let ff = 20;
console.assert((ff **= 3) === 8000);
