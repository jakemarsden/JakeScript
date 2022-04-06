console.assertEqual(findTriangleNumber(5), 28);

// Too slow to run by default when compiled without `--release`:
//console.assertEqual(findTriangleNumber(500), 76576500);

function findTriangleNumber(minDivisorCount) {
    let idx = 1;
    let tri = 1;
    while (tri <= minDivisorCount) {
        idx += 1;
        tri += idx;
    }
    while (countDivisorsOf(tri) < minDivisorCount) {
        idx += 1;
        tri += idx;
    }
    return tri;
}

function countDivisorsOf(n) {
    if (n <= 2) return n;

    // We already know that (1, n) are divisors of n.
    let count = 2;
    let midFactor = Math.sqrt(n);
    for (let factor = 2; factor < midFactor; factor += 1) {
        if (n % factor === 0) {
            count += 2;
        }
    }
    if (n % midFactor === 0) {
        count += 1;
    }
    return count;
}
