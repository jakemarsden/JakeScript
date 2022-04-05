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

assertUnmodifiable(function() { return Math.E; }, function(n) { return Math.E = n; });
assertUnmodifiable(function() { return Math.LN2; }, function(n) { return Math.LN2 = n; });
assertUnmodifiable(function() { return Math.LN10; }, function(n) { return Math.LN10 = n; });
assertUnmodifiable(function() { return Math.LOG2E; }, function(n) { return Math.LOG2E = n; });
assertUnmodifiable(function() { return Math.LOG10E; }, function(n) { return Math.LOG10E = n; });
assertUnmodifiable(function() { return Math.PI; }, function(n) { return Math.PI = n; });
assertUnmodifiable(function() { return Math.SQRT1_2; }, function(n) { return Math.SQRT1_2 = n; });
assertUnmodifiable(function() { return Math.SQRT2; }, function(n) { return Math.SQRT2 = n; });

function assertApproxEqual(actual, expected, msg) {
    console.assert(actual > expected && actual < expected + 1, msg);
}

function assertUnmodifiable(getter, setter) {
  let original = getter();
  console.assert(setter(42) === 42);
  let hopefullyUnmodified = getter();
  console.assert(hopefullyUnmodified === original);
  console.assert(hopefullyUnmodified !== 42);
}

console.assert(Math.abs);
console.assert(isNaN(Math.abs()));
console.assert(Math.abs(0) === 0);
console.assert(Math.abs(42) === 42);
console.assert(Math.abs(-42) === 42);
console.assert(Math.abs("") === 0);
console.assert(Math.abs("0") === 0);
console.assert(Math.abs("42") === 42);
console.assert(Math.abs("-42") === 42);
console.assert(isNaN(Math.abs(NaN)));
console.assert(isNaN(Math.abs("fish")));
console.assert(Math.abs(Infinity) === Infinity);
console.assert(Math.abs(-Infinity) === Infinity);

console.assert(Math.max);
console.assert(Math.max() === -Infinity);
console.assert(Math.max(42) === 42);
console.assert(Math.max(48, 93, 86, 68, 33) === 93);
console.assert(Math.max(48, 93, "86", 68, 33) === 93);
console.assert(isNaN(Math.max(48, 93, NaN, 68, 33)));
console.assert(isNaN(Math.max(48, 93, "fish", 68, 33)));
console.assert(Math.max(48, 93, Infinity, 68, 33) === Infinity);
console.assert(Math.max(48, 93, -Infinity, 68, 33) === 93);

console.assert(Math.min);
console.assert(Math.min() === Infinity);
console.assert(Math.min(42) === 42);
console.assert(Math.min(48, 93, 86, 68, 33) === 33);
console.assert(Math.min(48, 93, "86", 68, 33) === 33);
console.assert(isNaN(Math.min(48, 93, NaN, 68, 33)));
console.assert(isNaN(Math.min(48, 93, "fish", 68, 33)));
console.assert(Math.min(48, 93, Infinity, 68, 33) === 33);
console.assert(Math.min(48, 93, -Infinity, 68, 33) === -Infinity);

console.assert(Math.sqrt);
console.assert(Math.sqrt(1) === 1);
console.assert(Math.sqrt(4) === 2);
console.assert(Math.sqrt(256) === 16);
console.assert(Math.sqrt("256") === 16);
console.assert(isNaN(Math.sqrt(NaN)));
console.assert(isNaN(Math.sqrt("fish")));
console.assert(Math.sqrt(Infinity) === Infinity);
console.assert(isNaN(Math.sqrt(-Infinity)));

console.assert(Math.trunc);
console.assert(Math.trunc(Math.E) === 2);
console.assert(Math.trunc(Math.LN2) === 0);
console.assert(Math.trunc(Math.LN10) === 2);
console.assert(Math.trunc(Math.LOG2E) === 1);
console.assert(Math.trunc(Math.LOG10E) === 0);
console.assert(Math.trunc(Math.PI) === 3);
console.assert(Math.trunc(Math.SQRT1_2) === 0);
console.assert(Math.trunc(Math.SQRT2) === 1);
console.assert(Math.trunc("3") === 3);
console.assert(isNaN(Math.trunc(NaN)));
console.assert(isNaN(Math.trunc("fish")));
console.assert(Math.trunc(Infinity) === Infinity);
console.assert(Math.trunc(-Infinity) === -Infinity);
