let x = 0;
while (x < 3)
    x = x + 1;
console.assert(x === 3);

while (true) break;

let shouldBreak = false;
while (true)
    if (shouldBreak)
        break;
    else
        shouldBreak = true;
console.assert(shouldBreak === true);
