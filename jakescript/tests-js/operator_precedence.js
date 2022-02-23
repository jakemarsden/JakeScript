console.assert(30 === 10 + 20);
console.assert(10 + 20 === 30);

console.assert(50 + 100 + 17 === 167);
console.assert(2 + 3 * 4 === 14);
console.assert(2 * 3 + 4 === 10);

console.assert(100 * 7 + 3 === 703 && 100 * 7 + 3 !== 100 * 10);
console.assert(100 + 7 * 3 === 121 && 100 + 7 * 3 !== 300 + 21);

console.assert(1 + 2 + 3 * 4 * 5 + 6 * 7 + 8 === 8 - 5 * 3 + 120);
console.assert(8 - 5 * 3 + 120 === 1 + 2 + 3 * 4 * 5 + 6 * 7 + 8);

console.assert(3 ** 4 === 81);

console.assert(1 + 2 ** 3 * 4 ** 2 ** 2 + 6 === 2055);

console.assert(1 + 2 ** 3 * 4 === 33);
console.assert((1 + 2) ** 3 * 4 === 108);
console.assert(1 + (2 ** 3) * 4 === 33);
console.assert(1 + 2 ** (3 * 4) === 4097);
console.assert((1 + 2) ** (3 * 4) === 531441);
console.assert((1 + 2 ** 3) * 4 === 36);
console.assert(1 + (2 ** 3 * 4) === 33);
console.assert(((1 + 2) ** 3) * 4 === 108);
console.assert((1 + (2 ** 3)) * 4 === 36);
console.assert(1 + ((2 ** 3) * 4) === 33);
console.assert(1 + (2 ** (3 * 4)) === 4097);
console.assert((1 + 2 ** 3 * 4) === 33);
console.assert(((1 + 2 ** 3 * 4)) === 33);
console.assert((((1 + 2 ** 3 * 4))) === 33);

let a = 10;
console.assert(a++ * 2 === 20);
console.assert(a === 11);
console.assert(a-- * 2 === 22);
console.assert(a === 10);
console.assert(++a * 2 === 22);
console.assert(a === 11);
console.assert(--a * 2 === 20);
