let x = 0;
while (x < 3)
    x = x + 1;
console.assertEqual(x, 3);

let y = 0;
while (false)
    y = y + 1;
console.assertEqual(y, 0);

while (true) break;

let shouldBreak = false;
while (true)
    if (shouldBreak)
        break;
    else
        shouldBreak = true;
console.assertEqual(shouldBreak, true);

let emptyBody = false;
while (emptyBody);
