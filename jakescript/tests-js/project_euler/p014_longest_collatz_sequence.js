console.assertEqual(chainLength(13), 10);

// Too slow to run by default:
//console.assertEqual(longestCollatzChain(1000000), 837799);

function longestCollatzChain(maxStartNumber) {
    let maxSeed = 1;
    let maxChainLen = 0;
    for (let seed = 1; seed < maxStartNumber; seed += 1) {
        let chainLen = chainLength(seed);
        if (chainLen > maxChainLen) {
            maxSeed = seed;
            maxChainLen = chainLen;
        }
    }
    return maxSeed;
}

function chainLength(seed) {
    if (seed === 1) return 1;
    let n = seed;
    let len = 1;
    while (true) {
        while (n % 2 === 0) {
            n /= 2;
            len += 1;
            if (n === 1) return len;
        }
        n = 3 * n + 1;
        n /= 2;
        len += 2;
    }
}
