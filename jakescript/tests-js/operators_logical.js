console.assert((true && true) === true);
console.assert((true && false) === false);
console.assert((false && true) === false);
console.assert((false && false) === false);

console.assert((true || true) === true);
console.assert((true || false) === true);
console.assert((false || true) === true);
console.assert((false || false) === false);

console.assert(("lhs" && "rhs") === "rhs");
console.assert(("lhs" && "") === "");
console.assert(("" && "rhs") === "");
console.assert(("" && "") === "");

console.assert(("lhs" || "rhs") === "lhs");
console.assert(("lhs" || "") === "lhs");
console.assert(("" || "rhs") === "rhs");
console.assert(("" || "") === "");

function assertNotReached() {
    console.assert(false);
}

console.assert((false && assertNotReached()) === false);
console.assert((true || assertNotReached()) === true);
console.assert(("" && assertNotReached()) === "");
console.assert(("lhs" || assertNotReached()) === "lhs");

let counter = 0;

function condition(n, value) {
    counter += 1;
    console.assert(counter === n);
    return value;
}

function checkAndReset(expected) {
    console.assert(counter === expected);
    counter = 0;
}

console.assert((condition(1, true) && condition(2, true)) === true);
checkAndReset(2);
console.assert((condition(1, true) && condition(2, false)) === false);
checkAndReset(2);
console.assert((condition(1, false) && assertNotReached()) === false);
checkAndReset(1);
console.assert((condition(1, false) && assertNotReached()) === false);
checkAndReset(1);

console.assert((condition(1, true) || assertNotReached()) === true);
checkAndReset(1);
console.assert((condition(1, true) || assertNotReached()) === true);
checkAndReset(1);
console.assert((condition(1, false) || condition(2, true)) === true);
checkAndReset(2);
console.assert((condition(1, false) || condition(2, false)) === false);
checkAndReset(2);

console.assert((condition(1, 1) && condition(2, 2) && condition(3, 3) && condition(4, 4) && condition(5, 5)) === 5);
checkAndReset(5);
console.assert((condition(1, 1) && condition(2, 2) && condition(3, 3) && condition(4, 4) && condition(5, 0) && assertNotReached()) === 0);
checkAndReset(5);

console.assert((condition(1, 0) || condition(2, 0) || condition(3, 0) || condition(4, 0) || condition(5, 0)) === 0);
checkAndReset(5);
console.assert((condition(1, 0) || condition(2, 0) || condition(3, 0) || condition(4, 0) || condition(5, 5) || assertNotReached()) === 5);
checkAndReset(5);
