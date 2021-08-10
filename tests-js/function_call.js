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
earlyReturn(42);
assert earlyReturn(69) === "nice";
