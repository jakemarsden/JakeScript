let noArgCalled;
function noArg() {
    noArgCalled = true;
}
console.assertEqual(noArg(), undefined);
console.assert(noArgCalled);

let oneArgCalled;
function oneArg(a) {
    oneArgCalled = true;
    console.assertEqual(a, 10);
}
console.assertEqual(oneArg(10), undefined);
console.assert(oneArgCalled);

let multiArgCalled;
function multiArg(a, b, c) {
    multiArgCalled = true;
    console.assertEqual(a, b);
    console.assertEqual(a, c);
}
console.assertEqual(multiArg(2 + 2, 2 * 2, 2 ** 2), undefined);
console.assert(multiArgCalled);

let addCalled;
function add(a, b) {
    addCalled = true;
    return a + b;
}
console.assertEqual(add(10, 20), 30);
console.assert(addCalled);

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
console.assertEqual(earlyReturn(42), undefined);
console.assertEqual(earlyReturn(69), "nice");

function trailingExpressionIsNotReturned(a, b) {
    let sum = a + b;
    sum;
}
console.assertEqual(trailingExpressionIsNotReturned(40, 2), undefined);

function actuallyReturnsUndefined() {
    return undefined;
}
console.assertEqual(actuallyReturnsUndefined(), undefined);

let n = 10;
function addOutside(delta) {
    n += delta;
}
console.assertEqual(n, 10);
addOutside(3);
addOutside(n);
addOutside(4);
console.assertEqual(n, 30);

function moreArgsThanParams(a, b) {
    console.assertEqual(a, 1);
    console.assertEqual(b, 2);
}
console.assertEqual(moreArgsThanParams(1, 2, 3, 4), undefined);

function moreParamsThanArgs(a, b, c, d) {
    console.assertEqual(a, 1);
    console.assertEqual(b, 2);
    console.assertEqual(c, undefined);
    console.assertEqual(d, undefined);
}
console.assertEqual(moreParamsThanArgs(1, 2), undefined);

let paramsEvaluatedCount = 0;
function param(x) {
    paramsEvaluatedCount += 1;
    return x;
}

function moreArgsThanParams2(a, b) {
    console.assertEqual(a, 1);
    console.assertEqual(b, 2);
}
moreArgsThanParams2(param(1), param(2), param(3), param(4));
console.assertEqual(paramsEvaluatedCount, 4);
paramsEvaluatedCount = 0;

function moreParamsThanArgs2(a, b, c, d) {
    console.assertEqual(a, 1);
    console.assertEqual(b, 2);
    console.assertEqual(c, undefined);
    console.assertEqual(d, undefined);
}
moreParamsThanArgs2(param(1), param(2));
console.assertEqual(paramsEvaluatedCount, 2);
paramsEvaluatedCount = 0;
