console.assertEqual(findTriplet(), 31875000);

function findTriplet() {
    // a + b + c = 1000
    for (let a = 3; a < 1000; a += 1) {
        for (let b = a + 1; b < 1000 - a; b += 1) {
            if (matchesConstraint(a, b)) {
                let c = 1000 - a - b;
                return a * b * c;
            }
        }
    }
    console.assertNotReached();
}

/**
 *   a + b + c = 1000
 * ∴         c = 1000 - (a + b)
 *
 *   a^2 + b^2 = c^2
 * ∴ a^2 + b^2 = (1000 - (a + b))^2
 *             = 1000^2 - 2000(a + b) + (a + b)^2
 *             = 1x10^6 - 2000a - 2000b + a^2 + b^2 + 2ab
 *     -1x10^6 = -2000a - 2000b + 2ab
 *      5x10^5 = 1000a + 1000b - ab
 */
function matchesConstraint(a, b) {
    return 1000 * a + 1000 * b - a * b === 5 * 10 ** 5;
}
