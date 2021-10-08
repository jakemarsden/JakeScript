assert "a" + "" === "a";
assert "a" + "b" === "ab";
assert "Hello, " + "world" + "!" === "Hello, world!";

assert "abc" + 1 === "abc1";
assert "abc" + (2 + 3) === "abc5";
assert "abc" + 2 + 3 === "abc23";

let counter = 0;

function assertFirst(n) {
    assert counter % 2 === 0;
    counter += 1;
    return n;
}
function assertSecond(n) {
    assert counter % 2 === 1;
    counter += 1;
    return n;
}

assert assertFirst("abc") + assertSecond("def") === "abcdef";
assert counter === 2;
