console.assertEqual(powOf2NumericString(15), String(32768));
console.assertEqual(powDigitSum(15), 26);

// Too slow to run by default when compiled without `--release`:
//console.assertEqual(powDigitSum(1000), 1366);

function powDigitSum(exp) {
    let n = powOf2NumericString(exp);
    return sumDigits(n);
}

function sumDigits(n) {
    let sum = 0;
    n = String(n);
    for (let chIdx = 0; chIdx < n.length; chIdx += 1) {
        sum += Number(n.charAt(chIdx));
    }
    return sum;
}

function powOf2NumericString(exp) {
    let acc = String(1);
    for (let i = 0; i < exp; i += 1) {
        acc = addNumericString(acc, acc);
    }
    return acc;
}

function addNumericString(addend, accumulator) {
    while (accumulator.length < addend.length) {
        accumulator = "0" + accumulator;
    }

    let carry = 0;
    let addendIdx = addend.length - 1;
    for (let accumulatorIdx = accumulator.length - 1; accumulatorIdx >= 0; accumulatorIdx -= 1) {
        let accumulatorDigit = Number(accumulator.charAt(accumulatorIdx));
        let addendDigit;
        if (addendIdx >= 0) {
            addendDigit = Number(addend.charAt(addendIdx));
            addendIdx -= 1;
        } else {
            addendDigit = 0;
        }
        let digitSum = accumulatorDigit + addendDigit + carry;
        accumulator = setCharAt(accumulator, accumulatorIdx, digitSum % 10);
        carry = digitSum / 10;
    }
    if (carry) {
        accumulator = String(carry) + accumulator;
    }
    return accumulator;
}

function setCharAt(s, idx, ch) {
    ch = String(ch);
    return s.substring(0, idx) + ch + s.substring(idx + ch.length);
}
