let a = 42;
let b = 0;

if (true) {
    let foo = 1;
    b = b + 1;
    while (foo < 5) {
        let bar = 3;
        foo = foo + bar;
        assert a === 42;
        b = b + 1;
    }
    let bar;
    assert a === 42;
    assert foo === 7;
    b = b + 1;
} else {
    let foo = 888;
    b = 9999;
}
let foo = 3;
assert a === 42;
b = b + 1;

while (foo > 0) {
    let baz = 5;
    baz = baz - 1;
    assert baz === 4;

    let bar = foo - 1;
    foo = bar;
    assert a === 42;
    b = b + 1;
}
let bar = 1;
assert a === 42;
assert foo === 0;
assert bar === 1;
assert b === 8;
