assert 0b11 === 3;
assert 0B11 === 3;
assert 0o77 === 63;
assert 0O77 === 63;
assert 0xff === 255;
assert 0Xff === 255;
assert 0xFF === 255;
assert 0XFF === 255;

assert 0b00101010 === 42;
assert 0B00101010 === 42;
assert 0o52 === 42;
assert 0O52 === 42;
assert 0x2a === 42;
assert 0X2a === 42;
assert 0x2A === 42;
assert 0X2A === 42;

assert NaN !== 0;
assert NaN !== 1;
assert NaN !== 42;
assert NaN !== NaN;
assert NaN !== undefined;
