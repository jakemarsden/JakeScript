function multiplesOf3Or5(limit) {
    let sum = 0;
    for (let n = 1; n < limit; n += 1) {
        if (n % 3 === 0 || n % 5 === 0) {
            sum += n;
        }
    }
    return sum;
}

console.assert(multiplesOf3Or5(10) === 23);
console.assert(multiplesOf3Or5(1000) === 233168);
