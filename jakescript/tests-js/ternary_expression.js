function isEven(x) {
    return x % 2 === 0 ? "Hello" : "world";
}

console.assert(isEven(2) === "Hello");
console.assert(isEven(3) === "world");

let aCalled = false;
let bCalled = false;

function a(x) {
    aCalled = true;
    return x;
}
function b(x) {
    bCalled = true;
    return x;
}

let result1 = true ? a("a") : b("b");
console.assert(result1 === "a");
console.assert(aCalled);
console.assert(!bCalled);

aCalled = false;
bCalled = false;

let result2 = false ? a("a") : b("b");
console.assert(result2 === "b");
console.assert(!aCalled);
console.assert(bCalled);
