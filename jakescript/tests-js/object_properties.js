let obj = {};
assert obj;

obj.hello = "world";
let objProp = obj.hello;
assert objProp === "world";

let objWithNestedObj = {};
objWithNestedObj.nested = obj;
assert objWithNestedObj.nested.hello === "world";
