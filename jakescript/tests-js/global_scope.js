assert isNaN(7) === false;
assert isNaN(Infinity) === false;
assert isNaN(-Infinity) === false;
assert isNaN(NaN) === true;
assert isNaN(null) === true;
assert isNaN(undefined) === true;
assert isNaN(true) === true;
assert isNaN(false) === true;
assert isNaN("Hello, world!") === true;
assert isNaN({}) === true;
assert isNaN(function() {}) === true;
assert isNaN(isNaN) === true;
assert isNaN == isNaN;
assert isNaN === isNaN;

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
