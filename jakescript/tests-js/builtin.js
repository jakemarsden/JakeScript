console.assert(Infinity);
console.assert(NaN !== NaN);
console.assert(undefined === undefined);

// Using an assignment operator on these properties of the global object should happily evaluate to
// the value you would expect, without _actually_ changing the value of the property.

assertUnmodifiable(function () { return Infinity; }, function (n) { return Infinity = n; });
assertUnmodifiable(function () { return Infinity; }, function (n) { return Infinity = n; });
assertUnmodifiable(function () { return NaN; }, function (n) { return NaN = n; });
assertUnmodifiable(function () { return undefined; }, function (n) { return undefined = n; });

function assertUnmodifiable(getter, setter) {
    let original = getter();
    console.assert(setter(42) === 42);
    let hopefullyUnmodified = getter();
    console.assert(hopefullyUnmodified === original || isNaN(hopefullyUnmodified) && isNaN(original));
    console.assert(hopefullyUnmodified !== 42);
}
