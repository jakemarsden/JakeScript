function test1() {
    if (true) {
        console.assertEqual(a, undefined);
        var a;
    }
    a = 5;
    console.assertEqual(a, 5);
}

function test2() {
    if (true) {
        console.assertEqual(a, undefined);
        var a = 5;
    }
    console.assertEqual(a, 5);
}

function test3() {
    if (true) {
        if (true) {
            if (true) {
                if (true) {
                    console.assertEqual(a, undefined);
                    var a = 5;
                }
                console.assertEqual(a, 5);
            }
            console.assertEqual(a, 5);
        }
        console.assertEqual(a, 5);
    }
    console.assertEqual(a, 5);
}

test1();
test2();
test3();

function test4() {
    console.assertEqual(b, 5);
    b += 1;
}

var b = 5;
test4();
console.assertEqual(b, 6);
