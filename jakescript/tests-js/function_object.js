function f() {
    return "Hello";
}

function g() {
    return "world";
}

f.answer = 42;
f.functionAsProperty = g;
console.assert(f() === "Hello");
console.assert(f.answer === 42);
console.assert(f.functionAsProperty() === "world");

let f2 = f;
console.assert(f2() === "Hello");
console.assert(f2.answer === 42);
console.assert(f2.functionAsProperty() === "world");
console.assert(f() === "Hello");
console.assert(f.answer === 42);
console.assert(f.functionAsProperty() === "world");

function h() {
    return "computer";
}

f.answer += 1295;
f.functionAsProperty = h;
console.assert(f2.answer === 1337);
console.assert(f2.functionAsProperty() === "computer");
