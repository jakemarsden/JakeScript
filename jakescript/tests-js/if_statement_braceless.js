
let a = 42;

let b = null;
if (a === 42)
    b = "success block!";
console.assertEqual(b, "success block!");

let c = undefined;
if (a !== 42)
    c = "success block!";
console.assertEqual(c, undefined);

let d;
if (a === 42)
    d = "success block!";
else
    d = "else block!";
console.assertEqual(d, "success block!");

let e;
if (a !== 42)
    e = "success block!";
else
    e = "else block!";
console.assertEqual(e, "else block!");

let f;
if (a === 42)
    f = "success block!";
else if (a === 43)
    f = "else-if block!";
else
    f = "else block!";
console.assertEqual(f, "success block!");

let g;
if (a === 41)
    g = "success block!";
else if (a === 42)
    g = "else-if block!";
else
    g = "else block!";
console.assertEqual(g, "else-if block!");

let h;
if (a === 41)
    h = "success block!";
else if (a === 43)
    h = "else-if block!";
else
    h = "else block!";
console.assertEqual(h, "else block!");

let i;
if (a === 42)
    i = "success block!";
else {
    if (a === 43)
        i = "nested if block!";
    else
        i = "nested else block!";
}
console.assertEqual(i, "success block!");

let j;
if (a === 41)
    j = "success block!";
else {
    if (a === 42)
        j = "nested if block!";
    else
        j = "nested else block!";
}
console.assertEqual(j, "nested if block!");

let k;
if (a === 41)
    k = "success block!";
else {
    if (a === 43)
        k = "nested if block!";
    else
        k = "nested else block!";
}
console.assertEqual(k, "nested else block!");

let l;
if (a !== 41)
    if (a === 42)
        l = "nested if block!";
    else
        l = "nested else block!";
else
    l = "else block!";
console.assertEqual(l, "nested if block!");

let m;
if (a !== 41)
    if (a !== 42)
        m = "nested if block!";
    else
        m = "nested else block!";
else
    m = "else block!";
console.assertEqual(m, "nested else block!");

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

console.assertEqual(n, "nested if block!");

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

console.assertEqual(o, "nested else block!");

if (a === 42)
    var p = "if block!";
else {}
console.assertEqual(p, "if block!");

let emptyBody1 = false;
if (emptyBody1);

let emptyBody2 = true;
if (emptyBody2);
