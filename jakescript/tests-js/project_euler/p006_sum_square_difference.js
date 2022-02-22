function sumSquareDifference(n) {
    const squareOfSum = (n * (n + 1) / 2) ** 2;

    let sumOfSquares = 0;
    for (let i = 1; i <= n; i += 1) {
        sumOfSquares += i ** 2;
    }
    return squareOfSum - sumOfSquares;
}

console.assert(sumSquareDifference(10) === 2640);
console.assert(sumSquareDifference(100) === 25164150);
