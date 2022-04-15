console.assert(Math);

// TODO: Parse floating point literals, because this is just silly.
assertApproxEqual(Math.E, 2, "E");
assertApproxEqual(Math.LN2, 0, "LN2");
assertApproxEqual(Math.LN10, 2, "LN10");
assertApproxEqual(Math.LOG2E, 1, "LOG2E");
assertApproxEqual(Math.LOG10E, 0, "LOG10E");
assertApproxEqual(Math.PI, 3, "PI");
assertApproxEqual(Math.SQRT1_2, 0, "SQRT1_2");
assertApproxEqual(Math.SQRT2, 1, "SQRT2");

assertUnmodifiable(function () { return Math.E; }, function (n) { return Math.E = n; }, "E");
assertUnmodifiable(function () { return Math.LN2; }, function (n) { return Math.LN2 = n; }, "LN2");
assertUnmodifiable(function () { return Math.LN10; }, function (n) { return Math.LN10 = n; }, "LN10");
assertUnmodifiable(function () { return Math.LOG2E; }, function (n) { return Math.LOG2E = n; }, "LOG2E");
assertUnmodifiable(function () { return Math.LOG10E; }, function (n) { return Math.LOG10E = n; }, "LOG10E");
assertUnmodifiable(function () { return Math.PI; }, function (n) { return Math.PI = n; }, "PI");
assertUnmodifiable(function () { return Math.SQRT1_2; }, function (n) { return Math.SQRT1_2 = n; }, "SQRT1_2");
assertUnmodifiable(function () { return Math.SQRT2; }, function (n) { return Math.SQRT2 = n; }, "SQRT2");

function assertApproxEqual(actual, expected, msg) {
    console.assert(
        actual > expected && actual < expected + 1,
        "expected approximately", "'" + expected + "'", "but was", "'" + actual + "':", msg);
}

function assertUnmodifiable(getter, setter, msg) {
    let original = getter();
    console.assertEqual(setter(42), 42, msg);
    let hopefullyUnmodified = getter();
    console.assert(hopefullyUnmodified === original, msg);
    console.assert(hopefullyUnmodified !== 42, msg);
}

console.assert(Math.abs);
console.assertEqual(Math.abs(), NaN);
console.assertEqual(Math.abs(0), 0);
console.assertEqual(Math.abs(42), 42);
console.assertEqual(Math.abs(-42), 42);
console.assertEqual(Math.abs(""), 0);
console.assertEqual(Math.abs("0"), 0);
console.assertEqual(Math.abs("42"), 42);
console.assertEqual(Math.abs("-42"), 42);
console.assertEqual(Math.abs(NaN), NaN);
console.assertEqual(Math.abs("fish"), NaN);
console.assertEqual(Math.abs(Infinity), Infinity);
console.assertEqual(Math.abs(-Infinity), Infinity);

console.assert(Math.floor);
console.assertEqual(Math.floor(0), 0);
console.assertEqual(Math.floor(5), 5);
// TODO: Parse floating point numbers.
console.assertEqual(Math.floor(Number("5.95")), 5);
console.assertEqual(Math.floor(Number("5.05")), 5);
console.assertEqual(Math.floor(Number("-5.05")), -6);
console.assertEqual(Math.floor(Infinity), Infinity);
console.assertEqual(Math.floor(-Infinity), -Infinity);
console.assertEqual(Math.floor(NaN), NaN);
console.assertEqual(Math.floor(null), 0);
console.assertEqual(Math.floor(undefined), NaN);
console.assertEqual(Math.floor(), NaN);

console.assert(Math.max);
console.assertEqual(Math.max(), -Infinity);
console.assertEqual(Math.max(42), 42);
console.assertEqual(Math.max(48, 93, 86, 68, 33), 93);
console.assertEqual(Math.max(48, 93, "86", 68, 33), 93);
console.assertEqual(Math.max(48, 93, NaN, 68, 33), NaN);
console.assertEqual(Math.max(48, 93, "fish", 68, 33), NaN);
console.assertEqual(Math.max(48, 93, Infinity, 68, 33), Infinity);
console.assertEqual(Math.max(48, 93, -Infinity, 68, 33), 93);

console.assert(Math.min);
console.assertEqual(Math.min(), Infinity);
console.assertEqual(Math.min(42), 42);
console.assertEqual(Math.min(48, 93, 86, 68, 33), 33);
console.assertEqual(Math.min(48, 93, "86", 68, 33), 33);
console.assertEqual(Math.min(48, 93, NaN, 68, 33), NaN);
console.assertEqual(Math.min(48, 93, "fish", 68, 33), NaN);
console.assertEqual(Math.min(48, 93, Infinity, 68, 33), 33);
console.assertEqual(Math.min(48, 93, -Infinity, 68, 33), -Infinity);

console.assert(Math.sqrt);
console.assertEqual(Math.sqrt(1), 1);
console.assertEqual(Math.sqrt(4), 2);
console.assertEqual(Math.sqrt(256), 16);
console.assertEqual(Math.sqrt("256"), 16);
console.assertEqual(Math.sqrt(NaN), NaN);
console.assertEqual(Math.sqrt("fish"), NaN);
console.assertEqual(Math.sqrt(Infinity), Infinity);
console.assertEqual(Math.sqrt(-Infinity), NaN);

console.assert(Math.trunc);
console.assertEqual(Math.trunc(Math.E), 2);
console.assertEqual(Math.trunc(Math.LN2), 0);
console.assertEqual(Math.trunc(Math.LN10), 2);
console.assertEqual(Math.trunc(Math.LOG2E), 1);
console.assertEqual(Math.trunc(Math.LOG10E), 0);
console.assertEqual(Math.trunc(Math.PI), 3);
console.assertEqual(Math.trunc(Math.SQRT1_2), 0);
console.assertEqual(Math.trunc(Math.SQRT2), 1);
console.assertEqual(Math.trunc("3"), 3);
console.assertEqual(Math.trunc(NaN), NaN);
console.assertEqual(Math.trunc("fish"), NaN);
console.assertEqual(Math.trunc(Infinity), Infinity);
console.assertEqual(Math.trunc(-Infinity), -Infinity);
