console.assertEqual("a" + "", "a");
console.assertEqual("a" + "b", "ab");
console.assertEqual("Hello, " + "world" + "!", "Hello, world!");

console.assertEqual("abc" + 1, "abc1");
console.assertEqual("abc" + (2 + 3), "abc5");
console.assertEqual("abc" + 2 + 3, "abc23");

console.assertEqual("Hello, " + Infinity, "Hello, Infinity");
console.assertEqual(Infinity + ", world!", "Infinity, world!");
console.assertEqual("Hello, " + NaN, "Hello, NaN");
console.assertEqual(NaN + ", world!", "NaN, world!");
console.assertEqual("Hello, " + undefined, "Hello, undefined");
console.assertEqual(undefined + ", world!", "undefined, world!");
console.assertEqual("Hello, " + null, "Hello, null");
console.assertEqual(null + ", world!", "null, world!");

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

console.assertEqual(assertFirst("abc") + assertSecond("def"), "abcdef");
console.assertEqual(counter, 2);
