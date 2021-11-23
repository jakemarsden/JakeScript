function assertNotReached() {
    assert false;
}

let a;
try {
    a = 42;
} catch {
    assertNotReached();
}
assert a === 42;

let b, c;
try {
    b = 42;
    throw "example exception";
    assertNotReached();
} catch {
    c = 42;
}
assert b === 42;
assert c === 42;

let d, e;
try {
    d = 42;
    throw "example exception";
    assertNotReached();
} catch (ex) {
    e = 42;
    assert ex === "example exception";
}
assert d === 42;
assert e === 42;

let f, g, h;
try {
    try {
        f = 42;
        throw "example exception";
        assertNotReached();
    } catch (originalEx) {
        g = 42;
        throw "rethrown " + originalEx;
        assertNotReached();
    }
    assertNotReached();
} catch (rethrownEx) {
    h = 42;
    assert rethrownEx === "rethrown example exception";
}
assert f === 42;
assert g === 42;
assert h === 42;
