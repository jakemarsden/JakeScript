console.assert(sumPrimes(10) === 17);

// Too slow to run by default:
//console.assert(sumPrimes(2000000) === 142913828922);

function sumPrimes(maxPrime) {
    let n = 2;
    let sum = 0;
    while (n < maxPrime) {
        sum += n;
        n = nextPrime(n);
    }
    return sum;
}

function nextPrime(n) {
    while (true) {
        n += n % 2 + 1;
        if (isPrime(n)) return n;
    }
}

function isPrime(n) {
    if (n <= 1) return false;
    if (n === 2) return true;
    if (n % 2 === 0) return false;

    for (let potentialFactor = 3; potentialFactor <= Math.sqrt(n); potentialFactor += 2) {
        if (n % potentialFactor === 0) {
            return false;
        }
    }
    return true;
}
