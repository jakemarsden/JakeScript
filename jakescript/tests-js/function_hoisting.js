console.assert(square);
console.assert(square(5) === 25);

function square(x) {
    return x * x;
}

console.assert(square);
console.assert(square(6) === 36);

console.assert(cube(4) === 64);

function cube(n) {
    return cubeImpl();

    function cubeImpl() {
        return n ** 3;
    }
}
