let binaryNot = 20;
console.assert((~binaryNot) === 0 - 21);
console.assert(binaryNot === 20);

let logicalNot = true;
console.assert((!logicalNot) === false);
console.assert(logicalNot === true);
console.assert((!!logicalNot) === true);
console.assert((!!!logicalNot) === false);
console.assert(logicalNot === true);

let numericPlus = 20;
console.assert((+numericPlus) === 20);
console.assert(numericPlus === 20);

let numericNegate = 20;
console.assert((-numericNegate) === 0 - 20);
console.assert(numericNegate === 20);

let prefixIncrement = 20;
console.assert((++prefixIncrement) === 21);
console.assert(prefixIncrement === 21);

let prefixDecrement = 20;
console.assert((--prefixDecrement) === 19);
console.assert(prefixDecrement === 19);

let postfixIncrement = 20;
console.assert((postfixIncrement++) === 20);
console.assert(postfixIncrement === 21);

let postfixDecrement = 20;
console.assert((postfixDecrement--) === 20);
console.assert(postfixDecrement === 19);

console.assert((~Infinity) === -1);
console.assert((~(-Infinity)) === -1);
console.assert(isNaN(~NaN));
console.assert((+Infinity) === Infinity);
console.assert(isNaN(+NaN));
console.assert((-Infinity) === 0 - Infinity);
console.assert(isNaN(-NaN));
