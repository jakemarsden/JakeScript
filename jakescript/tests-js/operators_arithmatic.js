assert 20 + 10 === 30;
assert 20 - 10 === 10;
assert 20 * 10 === 200;
assert 20 / 10 === 2;
assert 20 % 7 === 6;
assert 20 ** 3 === 8000;

let counter = 0;

function assertFirst(n) {
    assert counter % 2 === 0;
    counter += 1;
    return n;
}
function assertSecond(n) {
    assert counter % 2 === 1;
    counter += 1;
    return n;
}

assert assertFirst(20) + assertSecond(10) === 30;
assert assertFirst(20) - assertSecond(10) === 10;
assert assertFirst(20) * assertSecond(10) === 200;
assert assertFirst(20) / assertSecond(10) === 2;
assert assertFirst(20) % assertSecond(7) === 6;
assert assertSecond(20) ** assertFirst(3) === 8000;
assert counter === 12;
