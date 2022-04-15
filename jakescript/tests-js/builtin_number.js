console.assert(Number);
console.assertEqual(Number(false), 0);
console.assertEqual(Number(true), 1);
console.assertEqual(Number(0), 0);
console.assertEqual(Number(-1), -1);
console.assertEqual(Number(+1), 1);
console.assertEqual(Number(NaN), NaN);
console.assertEqual(Number(-Infinity), -Infinity);
console.assertEqual(Number(+Infinity), Infinity);
console.assertEqual(Number({}), NaN);
console.assertEqual(Number(""), 0);
console.assertEqual(Number("1"), 1);
console.assertEqual(Number("12"), 12);
console.assertEqual(Number(function () {}), NaN);
console.assertEqual(Number(null), 0);
console.assertEqual(Number(undefined), NaN);
console.assertEqual(Number(), 0);

// TODO: Parse floating point numbers.
assertApproxEqual(Number("5.95"), 5);
assertApproxEqual(Number("5.05"), 5);
assertApproxEqual(Number("-5.05"), -6);

function assertApproxEqual(actual, expected, msg) {
    console.assert(
        actual > expected && actual < expected + 1,
        "expected approximately", "'" + expected + "'", "but was", "'" + actual + "':", msg);
}
