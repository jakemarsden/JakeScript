let x = 0;
while (x < 3) {
    x = x + 1;
}
console.assert(x === 3);

let i = 3;
let counter = 1;
while (i !== 0) {
    i = i - 1;
    counter *= 2;
    counter += 1;
}
console.assert(i === 0);
console.assert(counter === 15);

while (true) {
    break;
    console.assertNotReached();
}

let breakFlag = false;
let breakCounter = 0;
while (true) {
    breakCounter += 1;
    if (breakFlag) {
        break;
    }
    breakFlag = true;
}
console.assert(breakFlag === true);
console.assert(breakCounter === 2);

let z = 0;
while (z < 3) {
    z += 1;
    continue;
    console.assertNotReached();
}
console.assert(z === 3);

let continueIdx = 0;
let continueCounter = 0;
while (continueIdx < 10) {
    continueIdx += 1;
    if (continueCounter === 3) {
        continue;
    }
    continueCounter += 1;
}
console.assert(continueIdx === 10);
console.assert(continueCounter === 3);

let bcIdx = 10;
let bcCount = 0;
while (true) {
    bcIdx -= 1;
    if (bcIdx >= 7) {
        continue;
    }
    if (bcIdx === 2) {
        break;
    }
    bcCount += 1;
}
console.assert(bcIdx === 2);
console.assert(bcCount === 4);

function returnInsideWhileLoop() {
    let riwlCounter = 3;
    while (riwlCounter -= 1) {
        return riwlCounter;
    }
    console.assertNotReached();
}
console.assert(returnInsideWhileLoop() === 2);

function returnInsideWhileLoop2() {
    let riwl2Counter = 0;
    while (true) {
        if (riwl2Counter >= 10) {
            return riwl2Counter;
        }
        riwl2Counter += 1;
    }
    console.assertNotReached();
}
console.assert(returnInsideWhileLoop2() === 10);
