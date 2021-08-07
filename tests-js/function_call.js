function noArg() {
    assert true;
}
noArg();

function oneArg(a) {
    assert a === 10;
}
oneArg(10);

function twoArgs(a, b) {
    assert a === b;
}
twoArgs(1 + 3, 2 * 2);
