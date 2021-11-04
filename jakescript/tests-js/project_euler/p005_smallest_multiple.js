function smallestMultiple(maxDivisor) {
    let minDivisorToCheck = (maxDivisor / 2) + 1;
    let maxDivisorToCheck = maxDivisor - 1;
    for (let n = maxDivisor;; n += maxDivisor) {
        let divisibleByAll = true;
        for (let divisor = maxDivisorToCheck; divisor >= minDivisorToCheck; divisor -= 1) {
            if (n % divisor !== 0) {
                divisibleByAll = false;
                break;
            }
        }
        if (divisibleByAll) {
            return n;
        }
    }
}

assert smallestMultiple(10) === 2520;

// Too slow to run by default:
//assert smallestMultiple(20) === 232792560;
