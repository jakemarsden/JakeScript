console.assert(console);

console.assert(console.assert);
console.assert(true);
console.assert(true, "console.assert", "is broken");

console.assert(console.assertEqual);
console.assertEqual(true);
console.assertEqual(true, true);
console.assertEqual(true, true, "console.assertEqual", "is broken");

console.assert(console.assertNotReached);

console.assert(console.log);
console.log("console.log", "message");
