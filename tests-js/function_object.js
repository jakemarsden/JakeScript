function f() {
    return "Hello";
}

f.answer = 42;
assert f() === "Hello";
assert f.answer === 42;

let f2 = f;
assert f2() === "Hello";
assert f2.answer === 42;
assert f() === "Hello";
assert f.answer === 42;

f.answer += 1295;
assert f2.answer === 1337;
