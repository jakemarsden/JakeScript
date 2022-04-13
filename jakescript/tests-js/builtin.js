console.assert(Infinity, "Infinity");
console.assertEqual(Infinity, Infinity, "Infinity");
console.assert(NaN !== NaN, "NaN");
console.assertEqual(NaN, NaN, "NaN");
console.assertEqual(undefined, undefined, "undefined");

// Using an assignment operator on these properties of the global object should happily evaluate to
// the value you would expect, without _actually_ changing the value of the property.

assertUnmodifiable(function () { return Infinity; }, function (n) { return Infinity = n; }, "Infinity");
assertUnmodifiable(function () { return Infinity; }, function (n) { return Infinity = n; }, "Infinity");
assertUnmodifiable(function () { return NaN; }, function (n) { return NaN = n; }, "NaN");
assertUnmodifiable(function () { return undefined; }, function (n) { return undefined = n; }, "undefined");

function assertUnmodifiable(getter, setter, msg) {
    let original = getter();
    console.assertEqual(setter(42), 42, msg);
    let hopefullyUnmodified = getter();
    console.assertEqual(hopefullyUnmodified, original, msg);
    console.assert(hopefullyUnmodified !== 42, msg);
}
