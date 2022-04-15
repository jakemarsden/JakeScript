console.assertEqual(routeCount(2), 6);

// FIXME: Numeric overflow (should auto-convert to floating point?).
//console.assertEqual(routeCount(20), 137846528820);

function routeCount(gridSize) {
    let dividend = 1;
    for (let x = gridSize + 1; x <= 2 * gridSize; x += 1) {
        dividend *= x;
    }

    let divisor = 1;
    for (let x = 1; x <= gridSize; x += 1) {
        divisor *= x;
    }
    return dividend / divisor;
}
