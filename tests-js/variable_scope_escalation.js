function test1() {
    if (true) {
        assert a === undefined;
        var a;
    }
    a = 5;
    assert a === 5;
}

function test2() {
    if (true) {
        assert a === undefined;
        var a = 5;
    }
    assert a === 5;
}

function test3() {
    if (true) {
        if (true) {
            if (true) {
                if (true) {
                    assert a === undefined;
                    var a = 5;
                }
                assert a === 5;
            }
            assert a === 5;
        }
        assert a === 5;
    }
    assert a === 5;
}

test1();
test2();
test3();

function test4() {
    assert b === 5;
    b += 1;
}

var b = 5;
test4();
assert b === 6;
