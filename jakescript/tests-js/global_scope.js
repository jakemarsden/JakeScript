console.assert(Boolean);
console.assert(Boolean(false) === false);
console.assert(Boolean(true) === true);
console.assert(Boolean(0) === false);
console.assert(Boolean(-1) === true);
console.assert(Boolean(+1) === true);
console.assert(Boolean(NaN) === false);
console.assert(Boolean(-Infinity) === true);
console.assert(Boolean(+Infinity) === true);
console.assert(Boolean({}) === true);
console.assert(Boolean("") === false);
console.assert(Boolean("a") === true);
console.assert(Boolean("false") === true);
console.assert(Boolean(function () {}));
console.assert(Boolean(null) === false);
console.assert(Boolean(undefined) === false);
console.assert(Boolean() === false);

console.assert(Number);
console.assert(Number(false) === 0);
console.assert(Number(true) === 1);
console.assert(Number(0) === 0);
console.assert(Number(-1) === -1);
console.assert(Number(+1) === 1);
console.assert(isNaN(Number(NaN)));
console.assert(Number(-Infinity) === -Infinity);
console.assert(Number(+Infinity) === Infinity);
console.assert(isNaN(Number({})));
console.assert(Number("") === 0);
console.assert(Number("1") === 1);
console.assert(Number("12") === 12);
console.assert(isNaN(Number(function () {})));
console.assert(Number(null) === 0);
console.assert(isNaN(Number(undefined)));
console.assert(Number() === 0);

console.assert(isNaN);
console.assert(isNaN(7) === false);
console.assert(isNaN(Infinity) === false);
console.assert(isNaN(-Infinity) === false);
console.assert(isNaN(NaN) === true);
console.assert(isNaN(null) === true);
console.assert(isNaN(undefined) === true);
console.assert(isNaN(true) === true);
console.assert(isNaN(false) === true);
console.assert(isNaN("Hello, world!") === true);
console.assert(isNaN({}) === true);
console.assert(isNaN(function() {}) === true);
console.assert(isNaN(isNaN) === true);
console.assert(isNaN == isNaN);
console.assert(isNaN === isNaN);

// Using an assignment operator on properties of the global object should return normally with the value you would
// expect, without _actually_ changing the value of the property.

let undefined_ = undefined;
console.assert((undefined = 3) === 3);
console.assert(undefined === undefined_);
console.assert((undefined += " world!") === "undefined world!");
console.assert(undefined === undefined_);

let Infinity_ = Infinity;
console.assert((Infinity = 3) === 3);
console.assert(Infinity === Infinity_);
console.assert((Infinity += " world!") === "Infinity world!");
console.assert(Infinity === Infinity_);

console.assert((NaN = 3) === 3);
console.assert(isNaN(NaN));
console.assert((NaN += " world!") === "NaN world!");
console.assert(isNaN(NaN));
