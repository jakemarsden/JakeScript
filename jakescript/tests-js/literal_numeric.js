console.assert(0b11 === 3);
console.assert(0B11 === 3);
console.assert(0o77 === 63);
console.assert(0O77 === 63);
console.assert(0xff === 255);
console.assert(0Xff === 255);
console.assert(0xFF === 255);
console.assert(0XFF === 255);

console.assert(0b00101010 === 42);
console.assert(0B00101010 === 42);
console.assert(0o52 === 42);
console.assert(0O52 === 42);
console.assert(0x2a === 42);
console.assert(0X2a === 42);
console.assert(0x2A === 42);
console.assert(0X2A === 42);

console.assert(Infinity !== 0);
console.assert(Infinity !== 1);
console.assert(Infinity !== 42);
console.assert(Infinity === Infinity);
console.assert(Infinity !== -Infinity);
console.assert(-Infinity === -Infinity);
console.assert(Infinity !== undefined);

console.assert(NaN !== 0);
console.assert(NaN !== 1);
console.assert(NaN !== 42);
console.assert(NaN !== Infinity);
console.assert(NaN !== -Infinity);
console.assert(NaN !== NaN);
console.assert(NaN !== undefined);
