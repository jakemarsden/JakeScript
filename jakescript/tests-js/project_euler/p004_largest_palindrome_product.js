console.assert(largestPalindromeProduct(2) === 9009);

// Too slow to run by default when compiled without `--release`:
//console.assert(largestPalindromeProduct(3) === 906609);

function largestPalindromeProduct(digits) {
    console.assert(digits > 0);
    let min = 10 ** (digits - 1);
    let max = (10 ** digits) - 1;

    let maxProduct = 0;
    for (let lhs = min; lhs <= max; lhs += 1) {
        for (let rhs = lhs; rhs <= max; rhs += 1) {
            let product = lhs * rhs;
            if (product > maxProduct && isPalindrome(product)) {
                maxProduct = product;
            }
        }
    }
    return maxProduct;
}

function isPalindrome(n) {
    let str = String(n);
    let frontIdx = 0;
    let backIdx = str.length - 1;
    while (frontIdx < backIdx) {
        // TODO: Support receivers in function calls.
        //  E.g. `String.charAt(str, frontIdx)` should be `str.charAt(frontIdx)`.
        if (String.charAt(str, frontIdx) !== String.charAt(str, backIdx)) {
            return false;
        }
        frontIdx += 1;
        backIdx -= 1;
    }
    return true;
}
