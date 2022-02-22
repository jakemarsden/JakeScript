console.assert(exit);

function assertNotReached() {
    console.assert(false);
}

console.assert(1 + 2 === 3);

exit();
assertNotReached();
