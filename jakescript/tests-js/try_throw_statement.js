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

let i, j;
try {
    i = 42;
} finally {
    j = 42;
}
assert i === 42;
assert j === 42;

let k, l, m;
try {
    k = 42;
    throw "example exception";
    assertNotReached();
} catch (ex) {
    l = 42;
    assert ex === "example exception";
} finally {
    m = 42;
}
assert k === 42;
assert l === 42;
assert m === 42;

let n, o, p;
try {
    try {
        n = 42;
        throw "example exception";
        assertNotReached();
    } finally {
        o = 42;
    }
    assertNotReached();
} catch (ex) {
    p = 42;
    assert ex === "example exception";
}
assert n === 42;
assert o === 42;
assert p === 42;

let q, r, s, t, u, v;
try {
    try {
        try {
            try {
                q = 42;
                throw "example exception";
                assertNotReached();
            } finally {
                r = 42;
            }
            assertNotReached();
        } finally {
            s = 42;
        }
        assertNotReached();
    } finally {
        t = 42;
    }
    assertNotReached();
} catch (ex) {
    u = 42;
    assert ex === "example exception";
} finally {
    v = 42;
}
assert q === 42;
assert r === 42;
assert s === 42;
assert t === 42;
assert u === 42;
assert v === 42;

let w, x, y, z;
try {
    try {
        w = 42;
        throw "example exception";
        assertNotReached();
    } finally {
        x = 42;
        throw "example exception which hides the original exception";
        assertNotReached();
    }
} catch (ex) {
    y = 42;
    assert ex === "example exception which hides the original exception";
} finally {
    z = 42;
}
assert w === 42;
assert x === 42;
assert y === 42;
assert z === 42;

let aa, ab, ac, ad, ae, af;
try {
    try {
        try {
            try {
                aa = 42;
                throw 1;
                assertNotReached();
            } finally {
                ab = 42;
                throw 2;
                assertNotReached();
            }
            assertNotReached();
        } finally {
            ac = 42;
            throw 3;
            assertNotReached();
        }
        assertNotReached();
    } finally {
        ad = 42;
        throw 4;
        assertNotReached();
    }
    assertNotReached();
} catch (ex) {
    ae = 42;
    assert ex === 4;
} finally {
    af = 42;
}
assert aa === 42;
assert ab === 42;
assert ac === 42;
assert ad === 42;
assert ae === 42;
assert af === 42;

let ag, ah;
try {
    try {
        throw 1;
    } finally {
        try {
            throw 2;
        } catch (ex) {
            ag = 42;
            assert ex === 2;
        }
    }
    ah = 42;
} catch (ex) {
    assertNotReached();
}
assert ag === 42;
assert ah === 42;

let ai;
try {
    try {
        throw 1;
    } finally {
        try {
            throw 2;
        } catch {
            throw 3;
        }
        assertNotReached();
    }
    assertNotReached();
} catch (ex) {
    ai = 42;
    assert ex === 3;
}
assert ai === 42;
