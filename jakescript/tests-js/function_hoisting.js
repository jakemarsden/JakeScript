assert square;
assert square(5) === 25;

function square(x) {
    return x * x;
}

assert square;
assert square(6) === 36;

assert cube(4) === 64;

function cube(n) {
    return cubeImpl();

    function cubeImpl() {
        return n ** 3;
    }
}
