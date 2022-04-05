console.assertEqual(0 + 0, 0);
console.assertEqual(0 + 3, 3);
console.assertEqual(0 + (-3), -3);
console.assertEqual(0 + Infinity, Infinity);
console.assertEqual(0 + (-Infinity), -Infinity);
console.assertEqual(0 + NaN, NaN);

console.assertEqual(5 + 0, 5);
console.assertEqual(5 + 3, 8);
console.assertEqual(5 + (-3), 2);
console.assertEqual(5 + Infinity, Infinity);
console.assertEqual(5 + (-Infinity), -Infinity);
console.assertEqual(5 + NaN, NaN);

console.assertEqual((-5) + 0, -5);
console.assertEqual((-5) + 3, -2);
console.assertEqual((-5) + (-3), -8);
console.assertEqual((-5) + Infinity, Infinity);
console.assertEqual((-5) + (-Infinity), -Infinity);
console.assertEqual((-5) + NaN, NaN);

console.assertEqual(Infinity + 0, Infinity);
console.assertEqual(Infinity + 3, Infinity);
console.assertEqual(Infinity + (-3), Infinity);
console.assertEqual(Infinity + Infinity, Infinity);
console.assertEqual(Infinity + (-Infinity), NaN);
console.assertEqual(Infinity + NaN, NaN);

console.assertEqual((-Infinity) + 0, -Infinity);
console.assertEqual((-Infinity) + 3, -Infinity);
console.assertEqual((-Infinity) + (-3), -Infinity);
console.assertEqual((-Infinity) + Infinity, NaN);
console.assertEqual((-Infinity) + (-Infinity), -Infinity);
console.assertEqual((-Infinity) + NaN, NaN);

console.assertEqual(NaN + 0, NaN);
console.assertEqual(NaN + 3, NaN);
console.assertEqual(NaN + (-3), NaN);
console.assertEqual(NaN + Infinity, NaN);
console.assertEqual(NaN + (-Infinity), NaN);
console.assertEqual(NaN + NaN, NaN);

console.assertEqual(0 - 0, 0);
console.assertEqual(0 - 3, -3);
console.assertEqual(0 - (-3), 3);
console.assertEqual(0 - Infinity, -Infinity);
console.assertEqual(0 - (-Infinity), Infinity);
console.assertEqual(0 - NaN, NaN);

console.assertEqual(5 - 0, 5);
console.assertEqual(5 - 3, 2);
console.assertEqual(5 - (-3), 8);
console.assertEqual(5 - Infinity, -Infinity);
console.assertEqual(5 - (-Infinity), Infinity);
console.assertEqual(5 - NaN, NaN);

console.assertEqual((-5) - 0, -5);
console.assertEqual((-5) - 3, -8);
console.assertEqual((-5) - (-3), -2);
console.assertEqual((-5) - Infinity, -Infinity);
console.assertEqual((-5) - (-Infinity), Infinity);
console.assertEqual((-5) - NaN, NaN);

console.assertEqual(Infinity - 0, Infinity);
console.assertEqual(Infinity - 3, Infinity);
console.assertEqual(Infinity - (-3), Infinity);
console.assertEqual(Infinity - Infinity, NaN);
console.assertEqual(Infinity - (-Infinity), Infinity);
console.assertEqual(Infinity - NaN, NaN);

console.assertEqual((-Infinity) - 0, -Infinity);
console.assertEqual((-Infinity) - 3, -Infinity);
console.assertEqual((-Infinity) - (-3), -Infinity);
console.assertEqual((-Infinity) - Infinity, -Infinity);
console.assertEqual((-Infinity) - (-Infinity), NaN);
console.assertEqual((-Infinity) - NaN, NaN);

console.assertEqual(NaN - 0, NaN);
console.assertEqual(NaN - 3, NaN);
console.assertEqual(NaN - (-3), NaN);
console.assertEqual(NaN - Infinity, NaN);
console.assertEqual(NaN - (-Infinity), NaN);
console.assertEqual(NaN - NaN, NaN);

console.assertEqual(0 * 0, 0);
console.assertEqual(0 * 3, 0);
console.assertEqual(0 * (-3), 0);
console.assertEqual(0 * Infinity, NaN);
console.assertEqual(0 * (-Infinity), NaN);
console.assertEqual(0 * NaN, NaN);

console.assertEqual(5 * 0, 0);
console.assertEqual(5 * 3, 15);
console.assertEqual(5 * (-3), -15);
console.assertEqual(5 * Infinity, Infinity);
console.assertEqual(5 * (-Infinity), -Infinity);
console.assertEqual(5 * NaN, NaN);

console.assertEqual((-5) * 0, 0);
console.assertEqual((-5) * 3, -15);
console.assertEqual((-5) * (-3), 15);
console.assertEqual((-5) * Infinity, -Infinity);
console.assertEqual((-5) * (-Infinity), Infinity);
console.assertEqual((-5) * NaN, NaN);

console.assertEqual(Infinity * 0, NaN);
console.assertEqual(Infinity * 3, Infinity);
console.assertEqual(Infinity * (-3), -Infinity);
console.assertEqual(Infinity * Infinity, Infinity);
console.assertEqual(Infinity * (-Infinity), -Infinity);
console.assertEqual(Infinity * NaN, NaN);

console.assertEqual((-Infinity) * 0, NaN);
console.assertEqual((-Infinity) * 3, -Infinity);
console.assertEqual((-Infinity) * (-3), Infinity);
console.assertEqual((-Infinity) * Infinity, -Infinity);
console.assertEqual((-Infinity) * (-Infinity), Infinity);
console.assertEqual((-Infinity) * NaN, NaN);

console.assertEqual(NaN * 0, NaN);
console.assertEqual(NaN * 3, NaN);
console.assertEqual(NaN * (-3), NaN);
console.assertEqual(NaN * Infinity, NaN);
console.assertEqual(NaN * (-Infinity), NaN);
console.assertEqual(NaN * NaN, NaN);

console.assertEqual(0 / 0, NaN);
console.assertEqual(0 / 3, 0);
console.assertEqual(0 / (-3), 0);
console.assertEqual(0 / Infinity, 0);
console.assertEqual(0 / (-Infinity), 0);
console.assertEqual(0 / NaN, NaN);

console.assertEqual(15 / 0, Infinity);
console.assertEqual(15 / 3, 5);
console.assertEqual(15 / (-3), -5);
console.assertEqual(15 / Infinity, 0);
console.assertEqual(15 / (-Infinity), 0);
console.assertEqual(15 / NaN, NaN);

console.assertEqual((-15) / 0, -Infinity);
console.assertEqual((-15) / 3, -5);
console.assertEqual((-15) / (-3), 5);
console.assertEqual((-15) / Infinity, 0);
console.assertEqual((-15) / (-Infinity), 0);
console.assertEqual((-15) / NaN, NaN);

console.assertEqual(Infinity / 0, Infinity);
console.assertEqual(Infinity / 3, Infinity);
console.assertEqual(Infinity / (-3), -Infinity);
console.assertEqual(Infinity / Infinity, NaN);
console.assertEqual(Infinity / (-Infinity), NaN);
console.assertEqual(Infinity / NaN, NaN);

console.assertEqual((-Infinity) / 0, -Infinity);
console.assertEqual((-Infinity) / 3, -Infinity);
console.assertEqual((-Infinity) / (-3), Infinity);
console.assertEqual((-Infinity) / Infinity, NaN);
console.assertEqual((-Infinity) / (-Infinity), NaN);
console.assertEqual((-Infinity) / NaN, NaN);

console.assertEqual(NaN / 0, NaN);
console.assertEqual(NaN / 3, NaN);
console.assertEqual(NaN / (-3), NaN);
console.assertEqual(NaN / Infinity, NaN);
console.assertEqual(NaN / (-Infinity), NaN);
console.assertEqual(NaN / NaN, NaN);

console.assertEqual(0 % 0, NaN);
console.assertEqual(0 % 7, 0);
console.assertEqual(0 % (-7), 0);
console.assertEqual(0 % Infinity, 0);
console.assertEqual(0 % (-Infinity), 0);
console.assertEqual(0 % NaN, NaN);

console.assertEqual(20 % 0, NaN);
console.assertEqual(20 % 7, 6);
console.assertEqual(20 % (-7), 6);
console.assertEqual(20 % Infinity, 20);
console.assertEqual(20 % (-Infinity), 20);
console.assertEqual(20 % NaN, NaN);

console.assertEqual((-20) % 0, NaN);
console.assertEqual((-20) % 7, -6);
console.assertEqual((-20) % (-7), -6);
console.assertEqual((-20) % Infinity, -20);
console.assertEqual((-20) % (-Infinity), -20);
console.assertEqual((-20) % NaN, NaN);

console.assertEqual(Infinity % 0, NaN);
console.assertEqual(Infinity % 7, NaN);
console.assertEqual(Infinity % (-7), NaN);
console.assertEqual(Infinity % Infinity, NaN);
console.assertEqual(Infinity % (-Infinity), NaN);
console.assertEqual(Infinity % NaN, NaN);

console.assertEqual((-Infinity) % 0, NaN);
console.assertEqual((-Infinity) % 7, NaN);
console.assertEqual((-Infinity) % (-7), NaN);
console.assertEqual((-Infinity) % Infinity, NaN);
console.assertEqual((-Infinity) % (-Infinity), NaN);
console.assertEqual((-Infinity) % NaN, NaN);

console.assertEqual(NaN % 0, NaN);
console.assertEqual(NaN % 7, NaN);
console.assertEqual(NaN % (-7), NaN);
console.assertEqual(NaN % Infinity, NaN);
console.assertEqual(NaN % (-Infinity), NaN);
console.assertEqual(NaN % NaN, NaN);

console.assertEqual(0 ** 0, 1);
console.assertEqual(0 ** 3, 0);
console.assertEqual(0 ** (-3), Infinity);
console.assertEqual(0 ** Infinity, 0);
console.assertEqual(0 ** (-Infinity), Infinity);
console.assertEqual(0 ** NaN, NaN);

console.assertEqual(5 ** 0, 1);
console.assertEqual(5 ** 3, 125);
// TODO: Floating point.
//console.assertEqual(5 ** (-3), 0.008);
console.assertEqual(5 ** Infinity, Infinity);
console.assertEqual(5 ** (-Infinity), 0);
console.assertEqual(5 ** NaN, NaN);

console.assertEqual((-5) ** 0, 1);
console.assertEqual((-5) ** 3, -125);
// TODO: Floating point.
//console.assertEqual((-5) ** (-3), -0.008);
console.assertEqual((-5) ** Infinity, Infinity);
console.assertEqual((-5) ** (-Infinity), 0);
console.assertEqual((-5) ** NaN, NaN);

console.assertEqual(Infinity ** 0, 1);
console.assertEqual(Infinity ** 3, Infinity);
console.assertEqual(Infinity ** (-3), 0);
console.assertEqual(Infinity ** Infinity, Infinity);
console.assertEqual(Infinity ** (-Infinity), 0);
console.assertEqual(Infinity ** NaN, NaN);

console.assertEqual((-Infinity) ** 0, 1);
console.assertEqual((-Infinity) ** 3, -Infinity);
console.assertEqual((-Infinity) ** (-3), 0);
console.assertEqual((-Infinity) ** Infinity, Infinity);
console.assertEqual((-Infinity) ** (-Infinity), 0);
console.assertEqual((-Infinity) ** NaN, NaN);

console.assertEqual(NaN ** 0, 1);
console.assertEqual(NaN ** 3, NaN);
console.assertEqual(NaN ** (-3), NaN);
console.assertEqual(NaN ** Infinity, NaN);
console.assertEqual(NaN ** (-Infinity), NaN);
console.assertEqual(NaN ** NaN, NaN);

let counter = 0;
function assertFirst(n) {
    console.assert(counter % 2 === 0);
    counter += 1;
    return n;
}
function assertSecond(n) {
    console.assert(counter % 2 === 1);
    counter += 1;
    return n;
}

console.assertEqual(assertFirst(20) + assertSecond(10), 30);
console.assertEqual(assertFirst(20) - assertSecond(10), 10);
console.assertEqual(assertFirst(20) * assertSecond(10), 200);
console.assertEqual(assertFirst(20) / assertSecond(10), 2);
console.assertEqual(assertFirst(20) % assertSecond(7), 6);
console.assertEqual(assertSecond(20) ** assertFirst(3), 8000);
console.assertEqual(counter, 12);
