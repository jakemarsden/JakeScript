
let a = 42;

let b = null;
if (a === 42)
    b = "success block!";
console.assert(b === "success block!");

let c = undefined;
if (a !== 42)
    c = "success block!";
console.assert(c === undefined);

let d;
if (a === 42)
    d = "success block!";
else
    d = "else block!";
console.assert(d === "success block!");

let e;
if (a !== 42)
    e = "success block!";
else
    e = "else block!";
console.assert(e === "else block!");

let f;
if (a === 42)
    f = "success block!";
else if (a === 43)
    f = "else-if block!";
else
    f = "else block!";
console.assert(f === "success block!");

let g;
if (a === 41)
    g = "success block!";
else if (a === 42)
    g = "else-if block!";
else
    g = "else block!";
console.assert(g === "else-if block!");

let h;
if (a === 41)
    h = "success block!";
else if (a === 43)
    h = "else-if block!";
else
    h = "else block!";
console.assert(h === "else block!");

let i;
if (a === 42)
    i = "success block!";
else {
    if (a === 43)
        i = "nested if block!";
    else
        i = "nested else block!";
}
console.assert(i === "success block!");

let j;
if (a === 41)
    j = "success block!";
else {
    if (a === 42)
        j = "nested if block!";
    else
        j = "nested else block!";
}
console.assert(j === "nested if block!");

let k;
if (a === 41)
    k = "success block!";
else {
    if (a === 43)
        k = "nested if block!";
    else
        k = "nested else block!";
}
console.assert(k === "nested else block!");

let l;
if (a !== 41)
    if (a === 42)
        l = "nested if block!";
    else
        l = "nested else block!";
else
    l = "else block!";
console.assert(l === "nested if block!");

let m;
if (a !== 41)
    if (a !== 42)
        m = "nested if block!";
    else
        m = "nested else block!";
else
    m = "else block!";
console.assert(m === "nested else block!");

let n;
if (a === 41)
    n = "if block!";
else if (a === 42)
    if (a === 42)
        n = "nested if block!";
    else
        n = "nested else block!";
else
    n = "else block!";

console.assert(n === "nested if block!");

let o;
if (a === 41)
    o = "if block!";
else if (a === 42)
    if (a !== 42)
        o = "nested if block!";
    else
        o = "nested else block!";
else
    o = "else block!";

console.assert(o === "nested else block!");

if (a === 42)
    var p = "if block!";
else {}
console.assert(p === "if block!");
