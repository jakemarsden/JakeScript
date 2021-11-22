let empty = [];

let two = 2;
let numbers = [1, two, "foo", null, "bar"];

assert numbers[0] === 1;
assert numbers[1] === 2;
assert numbers[2] === "foo";
assert numbers[3] === null;
assert numbers[4] === "bar";
assert numbers[5] === undefined;
assert numbers[500] === undefined;

function square(n) {
    return n * n;
}

assert numbers[two] === "foo";
assert numbers[1 + 2] === null;
assert numbers[square(2)] === "bar";
assert numbers[500 + 1] === undefined;
