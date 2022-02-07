// TODO: Implement built-in `isNaN()` function
function isNaN(x) {
    return x !== x;
}

// Using an assignment operator on properties of the global object should return normally with the value you would
// expect, without _actually_ changing the value of the property.

let undefined_ = undefined;
assert (undefined = 3) === 3;
assert undefined === undefined_;
assert (undefined += " world!") === "undefined world!";
assert undefined === undefined_;

let Infinity_ = Infinity;
assert (Infinity = 3) === 3;
assert Infinity === Infinity_;
assert (Infinity += " world!") === "Infinity world!";
assert Infinity === Infinity_;

assert (NaN = 3) === 3;
assert isNaN(NaN);
assert (NaN += " world!") === "NaN world!";
assert isNaN(NaN);
