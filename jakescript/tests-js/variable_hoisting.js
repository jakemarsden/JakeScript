console.assertEqual(a, undefined);
a = 3;
console.assertEqual(a, 3);
var a;
console.assertEqual(a, 3);

// Variable declaration itself is hoisted, so the variable exists, but its initialiser is not
console.assertEqual(b, undefined);
var b = 3 + 4;
console.assertEqual(b, 7);
