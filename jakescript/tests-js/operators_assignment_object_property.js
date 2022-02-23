let obj = {};

obj.z = 20;
console.assert(obj.z === 20);

obj.a = 20;
obj.a += 10;
console.assert(obj.a === 30);
obj.b = 20;
obj.b -= 10;
console.assert(obj.b === 10);
obj.c = 20;
obj.c *= 10;
console.assert(obj.c === 200);
obj.d = 20;
obj.d /= 10;
console.assert(obj.d === 2);
obj.e = 20;
obj.e %= 7;
console.assert(obj.e === 6);
obj.f = 20;
obj.f **= 3;
console.assert(obj.f === 8000);

console.assert((obj.zz = 20) === 20);

obj.aa = 20;
console.assert((obj.aa += 10) === 30);
obj.bb = 20;
console.assert((obj.bb -= 10) === 10);
obj.cc = 20;
console.assert((obj.cc *= 10) === 200);
obj.dd = 20;
console.assert((obj.dd /= 10) === 2);
obj.ee = 20;
console.assert((obj.ee %= 7) === 6);
obj.ff = 20;
console.assert((obj.ff **= 3) === 8000);
