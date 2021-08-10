function noArg() {
    assert true;
}
noArg();

function oneArg(a) {
    assert a === 10;
}
oneArg(10);

function multiArg(a, b, c) {
    assert a === b;
    assert a === c;
}
multiArg(2 + 2, 2 * 2, 2 ** 2);

function add(a, b) {
    return a + b;
}
assert add(10, 20) === 30;

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
    assert false;
}
assert earlyReturn(42) === undefined;
assert earlyReturn(69) === "nice";

function trailingExpressionIsNotReturned(a, b) {
    let sum = a + b;
    sum;
}
assert trailingExpressionIsNotReturned(40, 2) === undefined;

function actuallyReturnsUndefined() {
    return undefined;
}
assert actuallyReturnsUndefined() === undefined;
