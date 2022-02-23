function noArg() {
    console.assert(true);
}
noArg();

function oneArg(a) {
    console.assert(a === 10);
}
oneArg(10);

function multiArg(a, b, c) {
    console.assert(a === b);
    console.assert(a === c);
}
multiArg(2 + 2, 2 * 2, 2 ** 2);

function add(a, b) {
    return a + b;
}
console.assert(add(10, 20) === 30);

function earlyReturn(arg) {
    if (arg === 42) {
        return;
    }
    if (arg > 50) {
        let nice = "nice";
        if (arg === 69) {
            return nice;
        }
    }
    console.assertNotReached();
}
console.assert(earlyReturn(42) === undefined);
console.assert(earlyReturn(69) === "nice");

function trailingExpressionIsNotReturned(a, b) {
    let sum = a + b;
    sum;
}
console.assert(trailingExpressionIsNotReturned(40, 2) === undefined);

function actuallyReturnsUndefined() {
    return undefined;
}
console.assert(actuallyReturnsUndefined() === undefined);

let n = 10;
function addOutside(delta) {
    n += delta;
}
console.assert(n === 10);
addOutside(3);
addOutside(n);
addOutside(4);
console.assert(n === 30);
