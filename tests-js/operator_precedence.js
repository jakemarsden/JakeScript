assert 30 === 10 + 20;
assert 10 + 20 === 30;

assert 100 * 7 + 3 === 703 && 100 * 7 + 3 !== 100 * 10;
assert 100 + 7 * 3 === 121 && 100 + 7 * 3 !== 300 + 21;

assert 1 + 2 + 3 * 4 * 5 + 6 * 7 + 8 === 8 - 5 * 3 + 120;
assert 8 - 5 * 3 + 120 === 1 + 2 + 3 * 4 * 5 + 6 * 7 + 8;

assert 3 ** 4 === 81;

assert 1 + 2 ** 3 * 4 ** 2 ** 2 + 6 === 2055;