let obj = {};
console.assert(obj);

obj.hello = "world";
let objProp = obj.hello;
console.assert(objProp === "world");

let objWithNestedObj = {};
objWithNestedObj.nested = obj;
console.assert(objWithNestedObj.nested.hello === "world");

let objLiteral = {
    foo: "Hello",
    bar: "world"
};
console.assert(objLiteral.foo === "Hello");
console.assert(objLiteral.bar === "world");

let objLiteralTrailingComma = {
    foo: "Hello",
    bar: "world",
};
console.assert(objLiteralTrailingComma.foo === "Hello");
console.assert(objLiteralTrailingComma.bar === "world");

let objLiteralSingleton = { foo: "Hello" };
console.assert(objLiteralSingleton.foo === "Hello");

let objLiteralSingletonTrailingComma = { foo: "Hello", };
console.assert(objLiteralSingletonTrailingComma.foo = "Hello");
