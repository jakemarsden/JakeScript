let binaryNot = 20;
assert (~binaryNot) === 0 - 21;
assert binaryNot === 20;

let logicalNot = true;
assert (!logicalNot) === false;
assert logicalNot === true;
assert (!!logicalNot) === true;
assert (!!!logicalNot) === false;
assert logicalNot === true;

let numericPlus = 20;
assert (+numericPlus) === 20;
assert numericPlus === 20;

let numericNegate = 20;
assert (-numericNegate) === 0 - 20;
assert numericNegate === 20;

let prefixIncrement = 20;
assert (++prefixIncrement) === 21;
assert prefixIncrement === 21;

let prefixDecrement = 20;
assert (--prefixDecrement) === 19;
assert prefixDecrement === 19;

let postfixIncrement = 20;
assert (postfixIncrement++) === 20;
assert postfixIncrement === 21;

let postfixDecrement = 20;
assert (postfixDecrement--) === 20;
assert postfixDecrement === 19;

assert (~Infinity) === -1;
assert (~(-Infinity)) === -1;
assert isNaN(~NaN);
assert (+Infinity) === Infinity;
assert isNaN(+NaN);
assert (-Infinity) === 0 - Infinity;
assert isNaN(-NaN);
