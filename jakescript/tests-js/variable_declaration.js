let a = 1000;
let b;
const c = 50;
var d = 10;
a = 13;
b = 100;
d = 4;
console.assertEqual(a + b + c + d, 167);

let foo, bar;
console.assertEqual(foo, undefined);
console.assertEqual(bar, undefined);

let baz = 1, qux = 2;
console.assertEqual(baz, 1);
console.assertEqual(qux, 2);
