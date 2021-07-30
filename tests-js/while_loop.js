let x = 0;
while (x < 3) {
    x = x + 1;
}
assert x === 3;

let i = 3;
let counter = 1;
while (i !== 0) {
    i = i - 1;
    counter *= 2;
    counter += 1;
}
assert i === 0;
assert counter === 15;
