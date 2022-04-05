console.assert("a" + "" === "a");
console.assert("a" + "b" === "ab");
console.assert("Hello, " + "world" + "!" === "Hello, world!");

console.assert("abc" + 1 === "abc1");
console.assert("abc" + (2 + 3) === "abc5");
console.assert("abc" + 2 + 3 === "abc23");

console.assert("Hello, " + Infinity === "Hello, Infinity");
console.assert(Infinity + ", world!" === "Infinity, world!");
console.assert("Hello, " + NaN === "Hello, NaN");
console.assert(NaN + ", world!" === "NaN, world!");
console.assert("Hello, " + undefined === "Hello, undefined");
console.assert(undefined + ", world!" === "undefined, world!");
console.assert("Hello, " + null === "Hello, null");
console.assert(null + ", world!" === "null, world!");

let counter = 0;

function assertFirst(n) {
    console.assert(counter % 2 === 0);
    counter += 1;
    return n;
}

function assertSecond(n) {
    console.assert(counter % 2 === 1);
    counter += 1;
    return n;
}

console.assert(assertFirst("abc") + assertSecond("def") === "abcdef");
console.assert(counter === 2);
