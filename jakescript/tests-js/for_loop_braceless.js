let counter = 0;
for (let i = 0; i <= 10; i += 1)
    counter += i;
console.assert(counter === 55);

let counterWithVar = 0;
for (var varI = 0; varI <= 10; varI += 1)
    counterWithVar += varI;
console.assert(counterWithVar === 55);

let noInitCounter = 0;
let noInitI = 5;
for (; noInitI > 0; noInitI -= 1)
    noInitCounter += 1;
console.assert(noInitI === 0);
console.assert(noInitCounter === 5);

function returnInside() {
    for (let i = 3; i -= 1;)
        return i;
    console.assertNotReached();
}
console.assert(returnInside() === 2);

function returnInside2() {
    for (let i = 0;; i += 1)
        if (i >= 10)
            return i;
    console.assertNotReached();
}
console.assert(returnInside2() === 10);
