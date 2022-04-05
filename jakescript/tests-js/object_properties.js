let obj = {};
console.assert(obj);

obj.hello = "world";
let objProp = obj.hello;
console.assertEqual(objProp, "world");

let objWithNestedObj = {};
objWithNestedObj.nested = obj;
console.assertEqual(objWithNestedObj.nested.hello, "world");

let objLiteral = {
    foo: "Hello",
    bar: "world"
};
console.assertEqual(objLiteral.foo, "Hello");
console.assertEqual(objLiteral.bar, "world");

let objLiteralTrailingComma = {
    foo: "Hello",
    bar: "world",
};
console.assertEqual(objLiteralTrailingComma.foo, "Hello");
console.assertEqual(objLiteralTrailingComma.bar, "world");

let objLiteralSingleton = { foo: "Hello" };
console.assertEqual(objLiteralSingleton.foo, "Hello");

let objLiteralSingletonTrailingComma = { foo: "Hello", };
console.assertEqual(objLiteralSingletonTrailingComma.foo, "Hello");
