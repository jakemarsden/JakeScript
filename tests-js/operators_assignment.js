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

// TODO: Support bracketed expressions (and remove this function).
function group(x) {
    return x;
}

let zz;
assert group(zz = 20) === 20;

let aa = 20;
assert group(aa += 10) === 30;
let bb = 20;
assert group(bb -= 10) === 10;
let cc = 20;
assert group(cc *= 10) === 200;
let dd = 20;
assert group(dd /= 10) === 2;
let ee = 20;
assert group(ee %= 7) === 6;
let ff = 20;
assert group(ff **= 3) === 8000;
