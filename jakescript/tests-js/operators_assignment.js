let z;
z = 20;
console.assertEqual(z, 20);

let a = 20;
a += 10;
console.assertEqual(a, 30);
let b = 20;
b -= 10;
console.assertEqual(b, 10);
let c = 20;
c *= 10;
console.assertEqual(c, 200);
let d = 20;
d /= 10;
console.assertEqual(d, 2);
let e = 20;
e %= 7;
console.assertEqual(e, 6);
let f = 20;
f **= 3;
console.assertEqual(f, 8000);

let zz;
console.assertEqual((zz = 20), 20);

let aa = 20;
console.assertEqual((aa += 10), 30);
let bb = 20;
console.assertEqual((bb -= 10), 10);
let cc = 20;
console.assertEqual((cc *= 10), 200);
let dd = 20;
console.assertEqual((dd /= 10), 2);
let ee = 20;
console.assertEqual((ee %= 7), 6);
let ff = 20;
console.assertEqual((ff **= 3), 8000);
