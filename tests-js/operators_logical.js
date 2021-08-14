assert (true && true) === true;
assert (true && false) === false;
assert (false && true) === false;
assert (false && false) === false;

assert (true || true) === true;
assert (true || false) === true;
assert (false || true) === true;
assert (false || false) === false;

function assertNotReached() {
    assert false;
}

assert (false && assertNotReached()) === false;
assert (true || assertNotReached()) === true;

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

assert (assertFirst(true) && assertSecond(true)) === true;
assert (assertFirst(true) && assertSecond(false)) === false;
assert (assertFirst(false) || assertSecond(true)) === true;
assert (assertFirst(false) || assertSecond(false)) === false;
assert counter === 8;
