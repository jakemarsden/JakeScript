assert Boolean;
assert Boolean(false) === false;
assert Boolean(true) === true;
assert Boolean(0) === false;
assert Boolean(-1) === true;
assert Boolean(+1) === true;
assert Boolean(NaN) === false;
assert Boolean(-Infinity) === true;
assert Boolean(+Infinity) === true;
assert Boolean({}) === true;
assert Boolean("") === false;
assert Boolean("a") === true;
assert Boolean("false") === true;
assert Boolean(function () {});
assert Boolean(null) === false;
assert Boolean(undefined) === false;
assert Boolean() === false;

assert Number;
assert Number(false) === 0;
assert Number(true) === 1;
assert Number(0) === 0;
assert Number(-1) === -1;
assert Number(+1) === 1;
assert isNaN(Number(NaN));
assert Number(-Infinity) === -Infinity;
assert Number(+Infinity) === Infinity;
assert isNaN(Number({}));
assert Number("") === 0;
assert Number("1") === 1;
assert Number("12") === 12;
assert isNaN(Number(function () {}));
assert Number(null) === 0;
assert isNaN(Number(undefined));
assert Number() === 0;

assert String;
assert String(false) === "false";
assert String(true) === "true";
assert String(0) === "0";
assert String(-1) === "-1";
assert String(+1) === "1";
assert String(NaN) === "NaN";
assert String(-Infinity) === "-Infinity";
assert String(+Infinity) === "Infinity";
assert String({}) === "[object Object]";
assert String("") === "";
assert String("a") === "a";
// TODO: assert String(function () {}) === "function () {}";
assert String(null) === "null";
assert String(undefined) === "undefined";
assert String() === "";

assert isNaN;
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
