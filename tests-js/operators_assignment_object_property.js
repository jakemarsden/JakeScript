let obj = {};

obj.z = 20;
assert obj.z === 20;

obj.a = 20;
obj.a += 10;
assert obj.a === 30;
obj.b = 20;
obj.b -= 10;
assert obj.b === 10;
obj.c = 20;
obj.c *= 10;
assert obj.c === 200;
obj.d = 20;
obj.d /= 10;
assert obj.d === 2;
obj.e = 20;
obj.e %= 7;
assert obj.e === 6;
obj.f = 20;
obj.f **= 3;
assert obj.f === 8000;

assert (obj.zz = 20) === 20;

obj.aa = 20;
assert (obj.aa += 10) === 30;
obj.bb = 20;
assert (obj.bb -= 10) === 10;
obj.cc = 20;
assert (obj.cc *= 10) === 200;
obj.dd = 20;
assert (obj.dd /= 10) === 2;
obj.ee = 20;
assert (obj.ee %= 7) === 6;
obj.ff = 20;
assert (obj.ff **= 3) === 8000;
