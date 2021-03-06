let counter = 0;
for (let i = 0; i <= 10; i += 1) {
    counter += i;
}
console.assertEqual(counter, 55);

let counterWithVar = 0;
for (var varI = 0; varI <= 10; varI += 1) {
    counterWithVar += varI;
}
console.assertEqual(counterWithVar, 55);

let noInitCounter = 0;
let noInitI = 5;
for (; noInitI > 0; noInitI -= 1) {
    noInitCounter += 1;
}
console.assertEqual(noInitI, 0);
console.assertEqual(noInitCounter, 5);

let noIncCounter = 0;
for (let i = 5; i > 0;) {
    i -= 1;
    noIncCounter += 1;
}
console.assertEqual(noIncCounter, 5);

let noCondCounter = 0;
for (let i = 5; ; i -= 1) {
    if (i <= 0) {
        break;
    }
    noCondCounter += 1;
}
console.assertEqual(noCondCounter, 5);

let infCounter = 0;
for (;;) {
    if (infCounter > 3) {
        break;
    }
    infCounter += 1;
}
console.assertEqual(infCounter, 4);

let breakCounter = 0;
for (let i = 0; i <= 10; i += 1) {
    if (i > 5) {
        break;
    }
    breakCounter += i;
}
console.assertEqual(breakCounter, 15);

let continueCounter = 0;
for (let i = 0; i < 10; i += 1) {
    if (i % 2 === 0) {
        continue;
    }
    continueCounter += i;
}
console.assertEqual(continueCounter, 25);

let bcCount = 0;
for (let idx = 9;; idx -= 1) {
    if (idx >= 7) {
        continue;
    }
    if (idx === 2) {
        break;
    }
    bcCount += 1;
}
console.assertEqual(bcCount, 4);

function returnInside() {
    for (let i = 3; i -= 1;) {
        return i;
    }
    console.assertNotReached();
}

console.assertEqual(returnInside(), 2);

function returnInside2() {
    for (let i = 0;; i += 1) {
        if (i >= 10) {
            return i;
        }
    }
    console.assertNotReached();
}

console.assertEqual(returnInside2(), 10);

let exprAsInitialiser1Idx = false;
for (1 + 1; !exprAsInitialiser1Idx; exprAsInitialiser1Idx = true) {}
console.assert(exprAsInitialiser1Idx);

let exprAsInitialiser2 = 0;
let exprAsInitialiser2Idx = 0;
for (exprAsInitialiser2 += 1; exprAsInitialiser2Idx < 3; exprAsInitialiser2Idx += 1) {}
console.assertEqual(exprAsInitialiser2, 1);
console.assertEqual(exprAsInitialiser2Idx, 3);

let exprAsInitialiser5Idx = 0;
let exprAsInitialiser5Counter = 0;
for (false; exprAsInitialiser5Idx < 3; exprAsInitialiser5Idx += 1) {
    exprAsInitialiser5Counter += 1;
}
console.assertEqual(exprAsInitialiser5Idx, 3);
console.assertEqual(exprAsInitialiser5Counter, 3);

let exprAsInitialiser4 = 0;
for (exprAsInitialiser4 += 1; false;) {
    console.assertNotReached();
}
console.assertEqual(exprAsInitialiser4, 1);

let exprAsInitialiser5Called = 0;
function expressionAsInitialiser5() {
    exprAsInitialiser5Called += 1;
}
for (expressionAsInitialiser5(); false;) {
    console.assertNotReached();
}
console.assertEqual(exprAsInitialiser5Called, 1);
