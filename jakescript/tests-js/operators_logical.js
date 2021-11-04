assert (true && true) === true;
assert (true && false) === false;
assert (false && true) === false;
assert (false && false) === false;

assert (true || true) === true;
assert (true || false) === true;
assert (false || true) === true;
assert (false || false) === false;

assert ("lhs" && "rhs") === "rhs";
assert ("lhs" && "") === "";
assert ("" && "rhs") === "";
assert ("" && "") === "";

assert ("lhs" || "rhs") === "lhs";
assert ("lhs" || "") === "lhs";
assert ("" || "rhs") === "rhs";
assert ("" || "") === "";

function assertNotReached() {
    assert false;
}

assert (false && assertNotReached()) === false;
assert (true || assertNotReached()) === true;
assert ("" && assertNotReached()) === "";
assert ("lhs" || assertNotReached()) === "lhs";

let counter = 0;

function condition(n, value) {
    counter += 1;
    assert counter === n;
    return value;
}

function checkAndReset(expected) {
    assert counter === expected;
    counter = 0;
}

assert (condition(1, true) && condition(2, true)) === true;
checkAndReset(2);
assert (condition(1, true) && condition(2, false)) === false;
checkAndReset(2);
assert (condition(1, false) && assertNotReached()) === false;
checkAndReset(1);
assert (condition(1, false) && assertNotReached()) === false;
checkAndReset(1);

assert (condition(1, true) || assertNotReached()) === true;
checkAndReset(1);
assert (condition(1, true) || assertNotReached()) === true;
checkAndReset(1);
assert (condition(1, false) || condition(2, true)) === true;
checkAndReset(2);
assert (condition(1, false) || condition(2, false)) === false;
checkAndReset(2);

assert (condition(1, 1) && condition(2, 2) && condition(3, 3) && condition(4, 4) && condition(5, 5)) === 5;
checkAndReset(5);
assert (condition(1, 1) && condition(2, 2) && condition(3, 3) && condition(4, 4) && condition(5, 0) && assertNotReached()) === 0;
checkAndReset(5);

assert (condition(1, 0) || condition(2, 0) || condition(3, 0) || condition(4, 0) || condition(5, 0)) === 0;
checkAndReset(5);
assert (condition(1, 0) || condition(2, 0) || condition(3, 0) || condition(4, 0) || condition(5, 5) || assertNotReached()) === 5;
checkAndReset(5);
