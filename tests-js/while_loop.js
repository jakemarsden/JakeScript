let x = 0;
while (x < 3) {
    x = x + 1;
}
assert x === 3;

let i = 3;
let counter = 1;
while (i !== 0) {
    i = i - 1;
    counter *= 2;
    counter += 1;
}
assert i === 0;
assert counter === 15;

while (true) {
    break;
    assert false;
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
assert breakFlag === true;
assert breakCounter === 2;

let z = 0;
while (z < 3) {
    z += 1;
    continue;
    assert false;
}
assert z === 3;

let continueIdx = 0;
let continueCounter = 0;
while (continueIdx < 10) {
    continueIdx += 1;
    if (continueCounter === 3) {
        continue;
    }
    continueCounter += 1;
}
assert continueIdx === 10;
assert continueCounter === 3;

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
assert bcIdx === 2;
assert bcCount === 4;
