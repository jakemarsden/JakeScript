let x = 0;
do
    x = x + 1;
while (x < 3);
console.assertEqual(x, 3);

let y = 0;
do
    y = y + 1;
while (false);
console.assertEqual(y, 1);

do break; while (console.assertNotReached());

let shouldBreak = false;
do
    if (shouldBreak)
        break;
    else
        shouldBreak = true;
while (true);
console.assertEqual(shouldBreak, true);
