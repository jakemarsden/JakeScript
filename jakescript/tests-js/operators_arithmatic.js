assert 0 + 0 === 0;
assert 0 + 3 === 3;
assert 0 + (-3) === -3;
assert 0 + Infinity === Infinity;
assert 0 + (-Infinity) === -Infinity;
assert isNaN(0 + NaN);

assert 5 + 0 === 5;
assert 5 + 3 === 8;
assert 5 + (-3) === 2;
assert 5 + Infinity === Infinity;
assert 5 + (-Infinity) === -Infinity;
assert isNaN(5 + NaN);

assert (-5) + 0 === -5;
assert (-5) + 3 === -2;
assert (-5) + (-3) === -8;
assert (-5) + Infinity === Infinity;
assert (-5) + (-Infinity) === -Infinity;
assert isNaN((-5) + NaN);

assert Infinity + 0 === Infinity;
assert Infinity + 3 === Infinity;
assert Infinity + (-3) === Infinity;
assert Infinity + Infinity === Infinity;
assert isNaN(Infinity + (-Infinity));
assert isNaN(Infinity + NaN);

assert (-Infinity) + 0 === -Infinity;
assert (-Infinity) + 3 === -Infinity;
assert (-Infinity) + (-3) === -Infinity;
assert isNaN((-Infinity) + Infinity);
assert (-Infinity) + (-Infinity) === -Infinity;
assert isNaN((-Infinity) + NaN);

assert isNaN(NaN + 0);
assert isNaN(NaN + 3);
assert isNaN(NaN + (-3));
assert isNaN(NaN + Infinity);
assert isNaN(NaN + (-Infinity));
assert isNaN(NaN + NaN);

assert 0 - 0 === 0;
assert 0 - 3 === -3;
assert 0 - (-3) === 3;
assert 0 - Infinity === -Infinity;
assert 0 - (-Infinity) === Infinity;
assert isNaN(0 - NaN);

assert 5 - 0 === 5;
assert 5 - 3 === 2;
assert 5 - (-3) === 8;
assert 5 - Infinity === -Infinity;
assert 5 - (-Infinity) === Infinity;
assert isNaN(5 - NaN);

assert (-5) - 0 === -5;
assert (-5) - 3 === -8;
assert (-5) - (-3) === -2;
assert (-5) - Infinity === -Infinity;
assert (-5) - (-Infinity) === Infinity;
assert isNaN((-5) - NaN);

assert Infinity - 0 === Infinity;
assert Infinity - 3 === Infinity;
assert Infinity - (-3) === Infinity;
assert isNaN(Infinity - Infinity);
assert Infinity - (-Infinity) === Infinity;
assert isNaN(Infinity - NaN);

assert (-Infinity) - 0 === -Infinity;
assert (-Infinity) - 3 === -Infinity;
assert (-Infinity) - (-3) === -Infinity;
assert (-Infinity) - Infinity === -Infinity;
assert isNaN((-Infinity) - (-Infinity));
assert isNaN((-Infinity) - NaN);

assert isNaN(NaN - 0);
assert isNaN(NaN - 3);
assert isNaN(NaN - (-3));
assert isNaN(NaN - Infinity);
assert isNaN(NaN - (-Infinity));
assert isNaN(NaN - NaN);

assert 0 * 0 === 0;
assert 0 * 3 === 0;
assert 0 * (-3) === 0;
assert isNaN(0 * Infinity);
assert isNaN(0 * (-Infinity));
assert isNaN(0 * NaN);

assert 5 * 0 === 0;
assert 5 * 3 === 15;
assert 5 * (-3) === -15;
assert 5 * Infinity === Infinity;
assert 5 * (-Infinity) === -Infinity;
assert isNaN(5 * NaN);

assert (-5) * 0 === 0;
assert (-5) * 3 === -15;
assert (-5) * (-3) === 15;
assert (-5) * Infinity === -Infinity;
assert (-5) * (-Infinity) === Infinity;
assert isNaN((-5) * NaN);

assert isNaN(Infinity * 0);
assert Infinity * 3 === Infinity;
assert Infinity * (-3) === -Infinity;
assert Infinity * Infinity === Infinity;
assert Infinity * (-Infinity) === -Infinity;
assert isNaN(Infinity * NaN);

assert isNaN((-Infinity) * 0);
assert (-Infinity) * 3 === -Infinity;
assert (-Infinity) * (-3) === Infinity;
assert (-Infinity) * Infinity === -Infinity;
assert (-Infinity) * (-Infinity) === Infinity;
assert isNaN((-Infinity) * NaN);

assert isNaN(NaN * 0);
assert isNaN(NaN * 3);
assert isNaN(NaN * (-3));
assert isNaN(NaN * Infinity);
assert isNaN(NaN * (-Infinity));
assert isNaN(NaN * NaN);

assert isNaN(0 / 0);
assert 0 / 3 === 0;
assert 0 / (-3) === 0;
assert 0 / Infinity === 0;
assert 0 / (-Infinity) === 0;
assert isNaN(0 / NaN);

assert 15 / 0 === Infinity;
assert 15 / 3 === 5;
assert 15 / (-3) === -5;
assert 15 / Infinity === 0;
assert 15 / (-Infinity) === 0;
assert isNaN(15 / NaN);

assert (-15) / 0 === -Infinity;
assert (-15) / 3 === -5;
assert (-15) / (-3) === 5;
assert (-15) / Infinity === 0;
assert (-15) / (-Infinity) === 0;
assert isNaN((-15) / NaN);

assert Infinity / 0 === Infinity;
assert Infinity / 3 === Infinity;
assert Infinity / (-3) === -Infinity;
assert isNaN(Infinity / Infinity);
assert isNaN(Infinity / (-Infinity));
assert isNaN(Infinity / NaN);

assert (-Infinity) / 0 === -Infinity;
assert (-Infinity) / 3 === -Infinity;
assert (-Infinity) / (-3) === Infinity;
assert isNaN((-Infinity) / Infinity);
assert isNaN((-Infinity) / (-Infinity));
assert isNaN((-Infinity) / NaN);

assert isNaN(NaN / 0);
assert isNaN(NaN / 3);
assert isNaN(NaN / (-3));
assert isNaN(NaN / Infinity);
assert isNaN(NaN / (-Infinity));
assert isNaN(NaN / NaN);

assert isNaN(0 % 0);
assert 0 % 7 === 0;
assert 0 % (-7) === 0;
assert 0 % Infinity === 0;
assert 0 % (-Infinity) === 0;
assert isNaN(0 % NaN);

assert isNaN(20 % 0);
assert 20 % 7 === 6;
assert 20 % (-7) === 6;
assert 20 % Infinity === 20;
assert 20 % (-Infinity) === 20;
assert isNaN(20 % NaN);

assert isNaN((-20) % 0);
assert (-20) % 7 === -6;
assert (-20) % (-7) === -6;
assert (-20) % Infinity === -20;
assert (-20) % (-Infinity) === -20;
assert isNaN((-20) % NaN);

assert isNaN(Infinity % 0);
assert isNaN(Infinity % 7);
assert isNaN(Infinity % (-7));
assert isNaN(Infinity % Infinity);
assert isNaN(Infinity % (-Infinity));
assert isNaN(Infinity % NaN);

assert isNaN((-Infinity) % 0);
assert isNaN((-Infinity) % 7);
assert isNaN((-Infinity) % (-7));
assert isNaN((-Infinity) % Infinity);
assert isNaN((-Infinity) % (-Infinity));
assert isNaN((-Infinity) % NaN);

assert isNaN(NaN % 0);
assert isNaN(NaN % 7);
assert isNaN(NaN % (-7));
assert isNaN(NaN % Infinity);
assert isNaN(NaN % (-Infinity));
assert isNaN(NaN % NaN);

assert 0 ** 0 === 1;
assert 0 ** 3 === 0;
assert 0 ** (-3) === Infinity;
assert 0 ** Infinity === 0;
assert 0 ** (-Infinity) === Infinity;
assert isNaN(0 ** NaN);

assert 5 ** 0 === 1;
assert 5 ** 3 === 125;
// TODO: Floating point.
//assert 5 ** (-3) === 0.008;
assert 5 ** Infinity === Infinity;
assert 5 ** (-Infinity) === 0;
assert isNaN(5 ** NaN);

assert (-5) ** 0 === 1;
assert (-5) ** 3 === -125;
// TODO: Floating point.
//assert (-5) ** (-3) === -0.008;
assert (-5) ** Infinity === Infinity;
assert (-5) ** (-Infinity) === 0;
assert isNaN((-5) ** NaN);

assert Infinity ** 0 === 1;
assert Infinity ** 3 === Infinity;
assert Infinity ** (-3) === 0;
assert Infinity ** Infinity === Infinity;
assert Infinity ** (-Infinity) === 0;
assert isNaN(Infinity ** NaN);

assert (-Infinity) ** 0 === 1;
assert (-Infinity) ** 3 === -Infinity;
assert (-Infinity) ** (-3) === 0;
assert (-Infinity) ** Infinity === Infinity;
assert (-Infinity) ** (-Infinity) === 0;
assert isNaN((-Infinity) ** NaN);

assert isNaN(NaN ** 0);
assert isNaN(NaN ** 3);
assert isNaN(NaN ** (-3));
assert isNaN(NaN ** Infinity);
assert isNaN(NaN ** (-Infinity));
assert isNaN(NaN ** NaN);

let counter = 0;
function assertFirst(n) {
    assert counter % 2 === 0;
    counter += 1;
    return n;
}
function assertSecond(n) {
    assert counter % 2 === 1;
    counter += 1;
    return n;
}

assert assertFirst(20) + assertSecond(10) === 30;
assert assertFirst(20) - assertSecond(10) === 10;
assert assertFirst(20) * assertSecond(10) === 200;
assert assertFirst(20) / assertSecond(10) === 2;
assert assertFirst(20) % assertSecond(7) === 6;
assert assertSecond(20) ** assertFirst(3) === 8000;
assert counter === 12;
