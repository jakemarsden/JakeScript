console.assert(nthPrime(6) === 13);

// Too slow to run by default when compiled without `--release`:
//console.assert(nthPrime(10001) === 104743);

function nthPrime(n) {
    let prime = 2;
    for (let idx = 1; idx < n; idx += 1) {
        prime = nextPrime(prime);
    }
    return prime;
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
