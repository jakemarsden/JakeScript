console.assert(largestPrimeFactor(13195) === 29);

// Too slow to run by default when compiled without `--release`:
//console.assert(largestPrimeFactor(600851475143) === 6857);

/**
 * @return `-1` if `n` has zero prime factors
 */
function largestPrimeFactor(n) {
    if (n <= 1) return -1;
    if (isPrime(n)) return n;

    let maxPrimeFactor = -1;
    let midFactor = Math.sqrt(n);
    for (let potentialFactor = 3; potentialFactor < midFactor; potentialFactor += 2) {
        if (!isDivisibleBy(n, potentialFactor)) continue;
        let factor1 = potentialFactor;
        let factor2 = n / factor1;

        let prime1 = largestPrimeFactor(factor1);
        if (prime1 > maxPrimeFactor) {
            maxPrimeFactor = prime1;
        }
        let prime2 = largestPrimeFactor(factor2);
        if (prime2 > maxPrimeFactor) {
            maxPrimeFactor = prime2;
        }
    }
    if (isDivisibleBy(n, midFactor)) {
        // `n` is a power of 2.
        let prime = largestPrimeFactor(midFactor);
        if (prime > maxPrimeFactor) {
            maxPrimeFactor = prime;
        }
    }
    return maxPrimeFactor;
}

function isPrime(n) {
    if (n <= 1) return false;
    if (n === 2) return true;
    if (isDivisibleBy(n, 2)) return false;

    for (let potentialFactor = 3; potentialFactor <= Math.sqrt(n); potentialFactor += 2) {
        if (isDivisibleBy(n, potentialFactor)) {
            return false;
        }
    }
    return true;
}

function isDivisibleBy(n, potentialFactor) {
    return n % potentialFactor === 0;
}
