console.assertEqual(letterCount(115), 20);
console.assertEqual(letterCount(342), 23);
console.assertEqual(letterCountUpTo(5), 19);
console.assertEqual(letterCountUpTo(1000), 21124);

function letterCountUpTo(n) {
    let count = 0;
    for (let i = 1; i <= n; i += 1) {
        count += letterCount(i);
    }
    return count;
}

function letterCount(n) {
    switch (n) {
    case 1:
    case 2:
    case 6:
    case 10:
        return 3;
    case 4:
    case 5:
    case 9:
        return 4;
    case 3:
    case 7:
    case 8:
    case 40:
    case 50:
    case 60:
        return 5;
    case 11:
    case 12:
    case 20:
    case 30:
    case 80:
    case 90:
        return 6;
    case 15:
    case 16:
    case 70:
        return 7;
    case 13:
    case 14:
    case 18:
    case 19:
        return 8;
    case 17:
        return 9;
    }

    let count = 0;
    let accountedForAnd = false;

    let thousands = Math.floor(n / 1000);
    if (thousands !== 0) {
        count += letterCount(thousands);
        count += "thousand".length;
        n %= 1000;

        if (n !== 0 && !accountedForAnd) {
            count += "and".length;
            accountedForAnd = true;
        }
    }

    let hundreds = Math.floor(n / 100);
    if (hundreds !== 0) {
        count += letterCount(hundreds);
        count += "hundred".length;
        n %= 100;

        if (n !== 0 && !accountedForAnd) {
            count += "and".length;
            accountedForAnd = true;
        }
    }

    if (n !== 0 && n < 20) {
        count += letterCount(n);
        return count;
    }

    let ones = n % 10;
    let tens = n - ones;
    if (tens !== 0) {
        count += letterCount(tens);
    }
    if (ones !== 0) {
        count += letterCount(ones);
    }

    return count;
}
