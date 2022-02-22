let empty = [];

let two = 2;
let numbers = [1, two, "foo", null, "bar"];

console.assert(numbers[0] === 1);
console.assert(numbers[1] === 2);
console.assert(numbers[2] === "foo");
console.assert(numbers[3] === null);
console.assert(numbers[4] === "bar");
console.assert(numbers[5] === undefined);
console.assert(numbers[500] === undefined);

function square(n) {
    return n * n;
}

console.assert(numbers[two] === "foo");
console.assert(numbers[1 + 2] === null);
console.assert(numbers[square(2)] === "bar");
console.assert(numbers[500 + 1] === undefined);
