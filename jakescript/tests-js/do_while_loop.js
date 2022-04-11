let x = 0;
do {
    x = x + 1;
} while (x < 3);
console.assertEqual(x, 3);

let y = 0;
do {
    y = y + 1;
} while (false);
console.assertEqual(y, 1);

let i = 3;
let counter = 1;
do {
    i = i - 1;
    counter *= 2;
    counter += 1;
} while (i !== 0);
console.assertEqual(i, 0);
console.assertEqual(counter, 15);

do {
    break;
    console.assertNotReached();
} while (console.assertNotReached());

let breakFlag = false;
let breakCounter = 0;
do {
    breakCounter += 1;
    if (breakFlag) {
        break;
    }
    breakFlag = true;
} while (true);
console.assertEqual(breakFlag, true);
console.assertEqual(breakCounter, 2);

let z = 0;
do {
    z += 1;
    if (z >= 3) {
        break;
    }
    continue;
    console.assertNotReached();
} while (console.assertNotReached());
console.assertEqual(z, 3);

let bcIdx = 10;
let bcCount = 0;
do {
    bcIdx -= 1;
    if (bcIdx >= 7) {
        continue;
    }
    if (bcIdx === 2) {
        break;
    }
    bcCount += 1;
} while (true);
console.assertEqual(bcIdx, 2);
console.assertEqual(bcCount, 4);

function returnInsideWhileLoop() {
    let riwlCounter = 3;
    do {
        return riwlCounter;
    } while (riwlCounter -= 1);
    console.assertNotReached();
}
console.assertEqual(returnInsideWhileLoop(), 3);

function returnInsideWhileLoop2() {
    let riwl2Counter = 0;
    do {
        if (riwl2Counter >= 10) {
            return riwl2Counter;
        }
        riwl2Counter += 1;
    } while (true);
    console.assertNotReached();
}
console.assertEqual(returnInsideWhileLoop2(), 10);
