console.assertEqual((true && true), true);
console.assertEqual((true && false), false);
console.assertEqual((false && true), false);
console.assertEqual((false && false), false);

console.assertEqual((true || true), true);
console.assertEqual((true || false), true);
console.assertEqual((false || true), true);
console.assertEqual((false || false), false);

console.assertEqual(("lhs" && "rhs"), "rhs");
console.assertEqual(("lhs" && ""), "");
console.assertEqual(("" && "rhs"), "");
console.assertEqual(("" && ""), "");

console.assertEqual(("lhs" || "rhs"), "lhs");
console.assertEqual(("lhs" || ""), "lhs");
console.assertEqual(("" || "rhs"), "rhs");
console.assertEqual(("" || ""), "");

console.assertEqual((false && console.assertNotReached()), false);
console.assertEqual((true || console.assertNotReached()), true);
console.assertEqual(("" && console.assertNotReached()), "");
console.assertEqual(("lhs" || console.assertNotReached()), "lhs");

let counter = 0;

function condition(n, value) {
    counter += 1;
    console.assertEqual(counter, n);
    return value;
}

function checkAndReset(expected) {
    console.assertEqual(counter, expected);
    counter = 0;
}

console.assertEqual((condition(1, true) && condition(2, true)), true);
checkAndReset(2);
console.assertEqual((condition(1, true) && condition(2, false)), false);
checkAndReset(2);
console.assertEqual((condition(1, false) && assertNotReached()), false);
checkAndReset(1);
console.assertEqual((condition(1, false) && assertNotReached()), false);
checkAndReset(1);

console.assertEqual((condition(1, true) || assertNotReached()), true);
checkAndReset(1);
console.assertEqual((condition(1, true) || assertNotReached()), true);
checkAndReset(1);
console.assertEqual((condition(1, false) || condition(2, true)), true);
checkAndReset(2);
console.assertEqual((condition(1, false) || condition(2, false)), false);
checkAndReset(2);

console.assertEqual((condition(1, 1) && condition(2, 2) && condition(3, 3) && condition(4, 4) && condition(5, 5)), 5);
checkAndReset(5);
console.assertEqual((condition(1, 1) && condition(2, 2) && condition(3, 3) && condition(4, 4) && condition(5, 0) && assertNotReached()), 0);
checkAndReset(5);

console.assertEqual((condition(1, 0) || condition(2, 0) || condition(3, 0) || condition(4, 0) || condition(5, 0)), 0);
checkAndReset(5);
console.assertEqual((condition(1, 0) || condition(2, 0) || condition(3, 0) || condition(4, 0) || condition(5, 5) || assertNotReached()), 5);
checkAndReset(5);
