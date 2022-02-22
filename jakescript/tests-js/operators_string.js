console.assert("a" + "" === "a");
console.assert("a" + "b" === "ab");
console.assert("Hello, " + "world" + "!" === "Hello, world!");

console.assert("abc" + 1 === "abc1");
console.assert("abc" + (2 + 3) === "abc5");
console.assert("abc" + 2 + 3 === "abc23");

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
