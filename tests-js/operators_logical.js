// TODO: Support bracketed expressions (and remove this functions). Currently required because the
//  the `===` operator takes precedence over the `&&` and `||` operators.
function group(x) {
    return x;
}

assert group(true && true) === true;
assert group(true && false) === false;
assert group(false && true) === false;
assert group(false && false) === false;

assert group(true || true) === true;
assert group(true || false) === true;
assert group(false || true) === true;
assert group(false || false) === false;

function assertNotReached() {
    assert false;
}

assert group(false && assertNotReached()) === false;
assert group(true || assertNotReached()) === true;

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

assert group(assertFirst(true) && assertSecond(true)) === true;
assert group(assertFirst(true) && assertSecond(false)) === false;
assert group(assertFirst(false) || assertSecond(true)) === true;
assert group(assertFirst(false) || assertSecond(false)) === false;
assert counter === 8;
