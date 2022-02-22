console.assert(0 + 0 === 0);
console.assert(0 + 3 === 3);
console.assert(0 + (-3) === -3);
console.assert(0 + Infinity === Infinity);
console.assert(0 + (-Infinity) === -Infinity);
console.assert(isNaN(0 + NaN));

console.assert(5 + 0 === 5);
console.assert(5 + 3 === 8);
console.assert(5 + (-3) === 2);
console.assert(5 + Infinity === Infinity);
console.assert(5 + (-Infinity) === -Infinity);
console.assert(isNaN(5 + NaN));

console.assert((-5) + 0 === -5);
console.assert((-5) + 3 === -2);
console.assert((-5) + (-3) === -8);
console.assert((-5) + Infinity === Infinity);
console.assert((-5) + (-Infinity) === -Infinity);
console.assert(isNaN((-5) + NaN));

console.assert(Infinity + 0 === Infinity);
console.assert(Infinity + 3 === Infinity);
console.assert(Infinity + (-3) === Infinity);
console.assert(Infinity + Infinity === Infinity);
console.assert(isNaN(Infinity + (-Infinity)));
console.assert(isNaN(Infinity + NaN));

console.assert((-Infinity) + 0 === -Infinity);
console.assert((-Infinity) + 3 === -Infinity);
console.assert((-Infinity) + (-3) === -Infinity);
console.assert(isNaN((-Infinity) + Infinity));
console.assert((-Infinity) + (-Infinity) === -Infinity);
console.assert(isNaN((-Infinity) + NaN));

console.assert(isNaN(NaN + 0));
console.assert(isNaN(NaN + 3));
console.assert(isNaN(NaN + (-3)));
console.assert(isNaN(NaN + Infinity));
console.assert(isNaN(NaN + (-Infinity)));
console.assert(isNaN(NaN + NaN));

console.assert(0 - 0 === 0);
console.assert(0 - 3 === -3);
console.assert(0 - (-3) === 3);
console.assert(0 - Infinity === -Infinity);
console.assert(0 - (-Infinity) === Infinity);
console.assert(isNaN(0 - NaN));

console.assert(5 - 0 === 5);
console.assert(5 - 3 === 2);
console.assert(5 - (-3) === 8);
console.assert(5 - Infinity === -Infinity);
console.assert(5 - (-Infinity) === Infinity);
console.assert(isNaN(5 - NaN));

console.assert((-5) - 0 === -5);
console.assert((-5) - 3 === -8);
console.assert((-5) - (-3) === -2);
console.assert((-5) - Infinity === -Infinity);
console.assert((-5) - (-Infinity) === Infinity);
console.assert(isNaN((-5) - NaN));

console.assert(Infinity - 0 === Infinity);
console.assert(Infinity - 3 === Infinity);
console.assert(Infinity - (-3) === Infinity);
console.assert(isNaN(Infinity - Infinity));
console.assert(Infinity - (-Infinity) === Infinity);
console.assert(isNaN(Infinity - NaN));

console.assert((-Infinity) - 0 === -Infinity);
console.assert((-Infinity) - 3 === -Infinity);
console.assert((-Infinity) - (-3) === -Infinity);
console.assert((-Infinity) - Infinity === -Infinity);
console.assert(isNaN((-Infinity) - (-Infinity)));
console.assert(isNaN((-Infinity) - NaN));

console.assert(isNaN(NaN - 0));
console.assert(isNaN(NaN - 3));
console.assert(isNaN(NaN - (-3)));
console.assert(isNaN(NaN - Infinity));
console.assert(isNaN(NaN - (-Infinity)));
console.assert(isNaN(NaN - NaN));

console.assert(0 * 0 === 0);
console.assert(0 * 3 === 0);
console.assert(0 * (-3) === 0);
console.assert(isNaN(0 * Infinity));
console.assert(isNaN(0 * (-Infinity)));
console.assert(isNaN(0 * NaN));

console.assert(5 * 0 === 0);
console.assert(5 * 3 === 15);
console.assert(5 * (-3) === -15);
console.assert(5 * Infinity === Infinity);
console.assert(5 * (-Infinity) === -Infinity);
console.assert(isNaN(5 * NaN));

console.assert((-5) * 0 === 0);
console.assert((-5) * 3 === -15);
console.assert((-5) * (-3) === 15);
console.assert((-5) * Infinity === -Infinity);
console.assert((-5) * (-Infinity) === Infinity);
console.assert(isNaN((-5) * NaN));

console.assert(isNaN(Infinity * 0));
console.assert(Infinity * 3 === Infinity);
console.assert(Infinity * (-3) === -Infinity);
console.assert(Infinity * Infinity === Infinity);
console.assert(Infinity * (-Infinity) === -Infinity);
console.assert(isNaN(Infinity * NaN));

console.assert(isNaN((-Infinity) * 0));
console.assert((-Infinity) * 3 === -Infinity);
console.assert((-Infinity) * (-3) === Infinity);
console.assert((-Infinity) * Infinity === -Infinity);
console.assert((-Infinity) * (-Infinity) === Infinity);
console.assert(isNaN((-Infinity) * NaN));

console.assert(isNaN(NaN * 0));
console.assert(isNaN(NaN * 3));
console.assert(isNaN(NaN * (-3)));
console.assert(isNaN(NaN * Infinity));
console.assert(isNaN(NaN * (-Infinity)));
console.assert(isNaN(NaN * NaN));

console.assert(isNaN(0 / 0));
console.assert(0 / 3 === 0);
console.assert(0 / (-3) === 0);
console.assert(0 / Infinity === 0);
console.assert(0 / (-Infinity) === 0);
console.assert(isNaN(0 / NaN));

console.assert(15 / 0 === Infinity);
console.assert(15 / 3 === 5);
console.assert(15 / (-3) === -5);
console.assert(15 / Infinity === 0);
console.assert(15 / (-Infinity) === 0);
console.assert(isNaN(15 / NaN));

console.assert((-15) / 0 === -Infinity);
console.assert((-15) / 3 === -5);
console.assert((-15) / (-3) === 5);
console.assert((-15) / Infinity === 0);
console.assert((-15) / (-Infinity) === 0);
console.assert(isNaN((-15) / NaN));

console.assert(Infinity / 0 === Infinity);
console.assert(Infinity / 3 === Infinity);
console.assert(Infinity / (-3) === -Infinity);
console.assert(isNaN(Infinity / Infinity));
console.assert(isNaN(Infinity / (-Infinity)));
console.assert(isNaN(Infinity / NaN));

console.assert((-Infinity) / 0 === -Infinity);
console.assert((-Infinity) / 3 === -Infinity);
console.assert((-Infinity) / (-3) === Infinity);
console.assert(isNaN((-Infinity) / Infinity));
console.assert(isNaN((-Infinity) / (-Infinity)));
console.assert(isNaN((-Infinity) / NaN));

console.assert(isNaN(NaN / 0));
console.assert(isNaN(NaN / 3));
console.assert(isNaN(NaN / (-3)));
console.assert(isNaN(NaN / Infinity));
console.assert(isNaN(NaN / (-Infinity)));
console.assert(isNaN(NaN / NaN));

console.assert(isNaN(0 % 0));
console.assert(0 % 7 === 0);
console.assert(0 % (-7) === 0);
console.assert(0 % Infinity === 0);
console.assert(0 % (-Infinity) === 0);
console.assert(isNaN(0 % NaN));

console.assert(isNaN(20 % 0));
console.assert(20 % 7 === 6);
console.assert(20 % (-7) === 6);
console.assert(20 % Infinity === 20);
console.assert(20 % (-Infinity) === 20);
console.assert(isNaN(20 % NaN));

console.assert(isNaN((-20) % 0));
console.assert((-20) % 7 === -6);
console.assert((-20) % (-7) === -6);
console.assert((-20) % Infinity === -20);
console.assert((-20) % (-Infinity) === -20);
console.assert(isNaN((-20) % NaN));

console.assert(isNaN(Infinity % 0));
console.assert(isNaN(Infinity % 7));
console.assert(isNaN(Infinity % (-7)));
console.assert(isNaN(Infinity % Infinity));
console.assert(isNaN(Infinity % (-Infinity)));
console.assert(isNaN(Infinity % NaN));

console.assert(isNaN((-Infinity) % 0));
console.assert(isNaN((-Infinity) % 7));
console.assert(isNaN((-Infinity) % (-7)));
console.assert(isNaN((-Infinity) % Infinity));
console.assert(isNaN((-Infinity) % (-Infinity)));
console.assert(isNaN((-Infinity) % NaN));

console.assert(isNaN(NaN % 0));
console.assert(isNaN(NaN % 7));
console.assert(isNaN(NaN % (-7)));
console.assert(isNaN(NaN % Infinity));
console.assert(isNaN(NaN % (-Infinity)));
console.assert(isNaN(NaN % NaN));

console.assert(0 ** 0 === 1);
console.assert(0 ** 3 === 0);
console.assert(0 ** (-3) === Infinity);
console.assert(0 ** Infinity === 0);
console.assert(0 ** (-Infinity) === Infinity);
console.assert(isNaN(0 ** NaN));

console.assert(5 ** 0 === 1);
console.assert(5 ** 3 === 125);
// TODO: Floating point.
//console.assert(5 ** (-3) === 0.008);
console.assert(5 ** Infinity === Infinity);
console.assert(5 ** (-Infinity) === 0);
console.assert(isNaN(5 ** NaN));

console.assert((-5) ** 0 === 1);
console.assert((-5) ** 3 === -125);
// TODO: Floating point.
//console.assert((-5) ** (-3) === -0.008);
console.assert((-5) ** Infinity === Infinity);
console.assert((-5) ** (-Infinity) === 0);
console.assert(isNaN((-5) ** NaN));

console.assert(Infinity ** 0 === 1);
console.assert(Infinity ** 3 === Infinity);
console.assert(Infinity ** (-3) === 0);
console.assert(Infinity ** Infinity === Infinity);
console.assert(Infinity ** (-Infinity) === 0);
console.assert(isNaN(Infinity ** NaN));

console.assert((-Infinity) ** 0 === 1);
console.assert((-Infinity) ** 3 === -Infinity);
console.assert((-Infinity) ** (-3) === 0);
console.assert((-Infinity) ** Infinity === Infinity);
console.assert((-Infinity) ** (-Infinity) === 0);
console.assert(isNaN((-Infinity) ** NaN));

console.assert(NaN ** 0 === 1);
console.assert(isNaN(NaN ** 3));
console.assert(isNaN(NaN ** (-3)));
console.assert(isNaN(NaN ** Infinity));
console.assert(isNaN(NaN ** (-Infinity)));
console.assert(isNaN(NaN ** NaN));

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

console.assert(assertFirst(20) + assertSecond(10) === 30);
console.assert(assertFirst(20) - assertSecond(10) === 10);
console.assert(assertFirst(20) * assertSecond(10) === 200);
console.assert(assertFirst(20) / assertSecond(10) === 2);
console.assert(assertFirst(20) % assertSecond(7) === 6);
console.assert(assertSecond(20) ** assertFirst(3) === 8000);
console.assert(counter === 12);
