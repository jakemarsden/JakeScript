let binaryNot = 20;
assert (~binaryNot) === 0 - 21;
assert binaryNot === 20;

let logicalNot = true;
assert (!logicalNot) === false;
assert logicalNot === true;

let numericPlus = 20;
assert (+numericPlus) === 20;
assert numericPlus === 20;

let numericNegate = 20;
assert (-numericNegate) === 0 - 20;
assert numericNegate === 20;
