function f() {
    return "Hello";
}

function g() {
    return "world";
}

f.answer = 42;
f.functionAsProperty = g;
console.assertEqual(f(), "Hello");
console.assertEqual(f.answer, 42);
console.assertEqual(f.functionAsProperty(), "world");

let f2 = f;
console.assertEqual(f2(), "Hello");
console.assertEqual(f2.answer, 42);
console.assertEqual(f2.functionAsProperty(), "world");
console.assertEqual(f(), "Hello");
console.assertEqual(f.answer, 42);
console.assertEqual(f.functionAsProperty(), "world");

function h() {
    return "computer";
}

f.answer += 1295;
f.functionAsProperty = h;
console.assertEqual(f2.answer, 1337);
console.assertEqual(f2.functionAsProperty(), "computer");
