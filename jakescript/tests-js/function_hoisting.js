console.assert(square);
console.assertEqual(square(5), 25);

function square(x) {
    return x * x;
}

console.assert(square);
console.assertEqual(square(6), 36);

console.assertEqual(cube(4), 64);

function cube(n) {
    return cubeImpl();

    function cubeImpl() {
        return n ** 3;
    }
}
