let counter = 0;
for (let i = 0; i <= 10; i += 1) {
    counter += i;
}
assert counter === 55;

let noInitCounter = 0;
let noInitI = 5;
for (; noInitI > 0; noInitI -= 1) {
    noInitCounter += 1;
}
assert noInitI === 0;
assert noInitCounter === 5;

let noIncCounter = 0;
for (let i = 5; i > 0;) {
    i -= 1;
    noIncCounter += 1;
}
assert noIncCounter === 5;

let noCondCounter = 0;
for (let i = 5; ; i -= 1) {
    if (i <= 0) {
        break;
    }
    noCondCounter += 1;
}
assert noCondCounter === 5;

let infCounter = 0;
for (;;) {
    if (infCounter > 3) {
        break;
    }
    infCounter += 1;
}
assert infCounter === 4;

let breakCounter = 0;
for (let i = 0; i <= 10; i += 1) {
    if (i > 5) {
        break;
    }
    breakCounter += i;
}
assert breakCounter === 15;

let continueCounter = 0;
for (let i = 0; i < 10; i += 1) {
    if (i % 2 === 0) {
        continue;
    }
    continueCounter += i;
}
assert continueCounter === 25;

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
assert bcCount === 4;

function returnInside() {
    for (let i = 3; i -= 1;) {
        return i;
    }
    assert false;
}
assert returnInside() === 2;

function returnInside2() {
    for (let i = 0;; i += 1) {
        if (i >= 10) {
            return i;
        }
    }
    assert false;
}
assert returnInside2() === 10;
