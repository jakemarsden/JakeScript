let a;
try {
    a = 42;
} catch {
    console.assertNotReached();
}
console.assertEqual(a, 42);

let b, c;
try {
    b = 42;
    throw "example exception";
    console.assertNotReached();
} catch {
    c = 42;
}
console.assertEqual(b, 42);
console.assertEqual(c, 42);

let d, e;
try {
    d = 42;
    throw "example exception";
    console.assertNotReached();
} catch (ex) {
    e = 42;
    console.assertEqual(ex, "example exception");
}
console.assertEqual(d, 42);
console.assertEqual(e, 42);

let f, g, h;
try {
    try {
        f = 42;
        throw "example exception";
        console.assertNotReached();
    } catch (originalEx) {
        g = 42;
        throw "rethrown " + originalEx;
        console.assertNotReached();
    }
    console.assertNotReached();
} catch (rethrownEx) {
    h = 42;
    console.assertEqual(rethrownEx, "rethrown example exception");
}
console.assertEqual(f, 42);
console.assertEqual(g, 42);
console.assertEqual(h, 42);

let i, j;
try {
    i = 42;
} finally {
    j = 42;
}
console.assertEqual(i, 42);
console.assertEqual(j, 42);

let k, l, m;
try {
    k = 42;
    throw "example exception";
    console.assertNotReached();
} catch (ex) {
    l = 42;
    console.assertEqual(ex, "example exception");
} finally {
    m = 42;
}
console.assertEqual(k, 42);
console.assertEqual(l, 42);
console.assertEqual(m, 42);

let n, o, p;
try {
    try {
        n = 42;
        throw "example exception";
        console.assertNotReached();
    } finally {
        o = 42;
    }
    console.assertNotReached();
} catch (ex) {
    p = 42;
    console.assertEqual(ex, "example exception");
}
console.assertEqual(n, 42);
console.assertEqual(o, 42);
console.assertEqual(p, 42);

let q, r, s, t, u, v;
try {
    try {
        try {
            try {
                q = 42;
                throw "example exception";
                console.assertNotReached();
            } finally {
                r = 42;
            }
            console.assertNotReached();
        } finally {
            s = 42;
        }
        console.assertNotReached();
    } finally {
        t = 42;
    }
    console.assertNotReached();
} catch (ex) {
    u = 42;
    console.assertEqual(ex, "example exception");
} finally {
    v = 42;
}
console.assertEqual(q, 42);
console.assertEqual(r, 42);
console.assertEqual(s, 42);
console.assertEqual(t, 42);
console.assertEqual(u, 42);
console.assertEqual(v, 42);

let w, x, y, z;
try {
    try {
        w = 42;
        throw "example exception";
        console.assertNotReached();
    } finally {
        x = 42;
        throw "example exception which hides the original exception";
        console.assertNotReached();
    }
} catch (ex) {
    y = 42;
    console.assertEqual(ex, "example exception which hides the original exception");
} finally {
    z = 42;
}
console.assertEqual(w, 42);
console.assertEqual(x, 42);
console.assertEqual(y, 42);
console.assertEqual(z, 42);

let aa, ab, ac, ad, ae, af;
try {
    try {
        try {
            try {
                aa = 42;
                throw 1;
                console.assertNotReached();
            } finally {
                ab = 42;
                throw 2;
                console.assertNotReached();
            }
            console.assertNotReached();
        } finally {
            ac = 42;
            throw 3;
            console.assertNotReached();
        }
        console.assertNotReached();
    } finally {
        ad = 42;
        throw 4;
        console.assertNotReached();
    }
    console.assertNotReached();
} catch (ex) {
    ae = 42;
    console.assertEqual(ex, 4);
} finally {
    af = 42;
}
console.assertEqual(aa, 42);
console.assertEqual(ab, 42);
console.assertEqual(ac, 42);
console.assertEqual(ad, 42);
console.assertEqual(ae, 42);
console.assertEqual(af, 42);

let ag, ah;
try {
    try {
        throw 1;
    } finally {
        try {
            throw 2;
        } catch (ex) {
            ag = 42;
            console.assertEqual(ex, 2);
        }
    }
    ah = 42;
} catch (ex) {
    console.assertNotReached();
}
console.assertEqual(ag, 42);
console.assertEqual(ah, 42);

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
        console.assertNotReached();
    }
    console.assertNotReached();
} catch (ex) {
    ai = 42;
    console.assertEqual(ex, 3);
}
console.assertEqual(ai, 42);
