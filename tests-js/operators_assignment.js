let z;
z = 20;
assert z === 20;

let a = 20;
a += 10;
assert a === 30;
let b = 20;
b -= 10;
assert b === 10;
let c = 20;
c *= 10;
assert c === 200;
let d = 20;
d /= 10;
assert d === 2;
let e = 20;
e %= 7;
assert e === 6;
let f = 20;
f **= 3;
assert f === 8000;

let zz;
assert (zz = 20) === 20;

let aa = 20;
assert (aa += 10) === 30;
let bb = 20;
assert (bb -= 10) === 10;
let cc = 20;
assert (cc *= 10) === 200;
let dd = 20;
assert (dd /= 10) === 2;
let ee = 20;
assert (ee %= 7) === 6;
let ff = 20;
assert (ff **= 3) === 8000;
