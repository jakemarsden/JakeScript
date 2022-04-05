let binaryNot = 20;
console.assertEqual((~binaryNot), 0 - 21);
console.assertEqual(binaryNot, 20);

let logicalNot = true;
console.assertEqual((!logicalNot), false);
console.assertEqual(logicalNot, true);
console.assertEqual((!!logicalNot), true);
console.assertEqual((!!!logicalNot), false);
console.assertEqual(logicalNot, true);

let numericPlus = 20;
console.assertEqual((+numericPlus), 20);
console.assertEqual(numericPlus, 20);

let numericNegate = 20;
console.assertEqual((-numericNegate), 0 - 20);
console.assertEqual(numericNegate, 20);

let prefixIncrement = 20;
console.assertEqual((++prefixIncrement), 21);
console.assertEqual(prefixIncrement, 21);

let prefixDecrement = 20;
console.assertEqual((--prefixDecrement), 19);
console.assertEqual(prefixDecrement, 19);

let postfixIncrement = 20;
console.assertEqual((postfixIncrement++), 20);
console.assertEqual(postfixIncrement, 21);

let postfixDecrement = 20;
console.assertEqual((postfixDecrement--), 20);
console.assertEqual(postfixDecrement, 19);

console.assertEqual((~Infinity), -1);
console.assertEqual((~(-Infinity)), -1);
console.assertEqual(~NaN, NaN);
console.assertEqual((+Infinity), Infinity);
console.assertEqual(+NaN, NaN);
console.assertEqual((-Infinity), 0 - Infinity);
console.assertEqual(-NaN, NaN);
