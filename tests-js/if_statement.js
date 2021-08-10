let a = 42;

let b = null;
if (a === 42) {
    b = "success block!";
}
assert b === "success block!";

let c = undefined;
if (a !== 42) {
    c = "success block!";
}
assert c === undefined;

let d;
if (a === 42) {
    d = "success block!";
} else {
    d = "else block!";
}
assert d === "success block!";

let e;
if (a !== 42) {
    e = "success block!";
} else {
    e = "else block!";
}
assert e === "else block!";
