let obj = {};
assert obj;

obj.hello = "world";
let objProp = obj.hello;
assert objProp === "world";

let objWithNestedObj = {};
objWithNestedObj.nested = obj;
assert objWithNestedObj.nested.hello === "world";

let objLiteral = {
    foo: "Hello",
    bar: "world"
};
assert objLiteral.foo === "Hello";
assert objLiteral.bar === "world";

let objLiteralTrailingComma = {
    foo: "Hello",
    bar: "world",
};
assert objLiteralTrailingComma.foo === "Hello";
assert objLiteralTrailingComma.bar === "world";

let objLiteralSingleton = { foo: "Hello" };
assert objLiteralSingleton.foo === "Hello";

let objLiteralSingletonTrailingComma = { foo: "Hello", };
assert objLiteralSingletonTrailingComma.foo = "Hello";
