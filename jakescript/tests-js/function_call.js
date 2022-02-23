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

function moreArgsThanParams(a, b) {
    console.assert(a === 1);
    console.assert(b === 2);
}
moreArgsThanParams(1, 2, 3, 4);

function moreParamsThanArgs(a, b, c, d) {
    console.assert(a === 1);
    console.assert(b === 2);
    console.assert(c === undefined);
    console.assert(d === undefined);
}
moreParamsThanArgs(1, 2);

let paramsEvaluatedCount = 0;
function param(x) {
    paramsEvaluatedCount += 1;
    return x;
}

function moreArgsThanParams2(a, b) {
    console.assert(a === 1);
    console.assert(b === 2);
}
moreArgsThanParams2(param(1), param(2), param(3), param(4));
console.assert(paramsEvaluatedCount === 4);
paramsEvaluatedCount = 0;

function moreParamsThanArgs2(a, b, c, d) {
    console.assert(a === 1);
    console.assert(b === 2);
    console.assert(c === undefined);
    console.assert(d === undefined);
}
moreParamsThanArgs2(param(1), param(2));
console.assert(paramsEvaluatedCount === 2);
paramsEvaluatedCount = 0;
