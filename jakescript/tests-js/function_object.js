function f() {
    return "Hello";
}

function g() {
    return "world";
}

f.answer = 42;
f.functionAsProperty = g;
assert f() === "Hello";
assert f.answer === 42;
assert f.functionAsProperty() === "world";

let f2 = f;
assert f2() === "Hello";
assert f2.answer === 42;
assert f2.functionAsProperty() === "world";
assert f() === "Hello";
assert f.answer === 42;
assert f.functionAsProperty() === "world";

function h() {
    return "computer";
}

f.answer += 1295;
f.functionAsProperty = h;
assert f2.answer === 1337;
assert f2.functionAsProperty() === "computer";
