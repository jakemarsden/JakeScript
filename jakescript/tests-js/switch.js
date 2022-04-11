let firstCaseNoDefault;
switch ("foo") {
    case "foo":
        firstCaseNoDefault = "foo case";
        break;
    case "bar":
        firstCaseNoDefault = "bar case";
        break;
}
console.assertEqual(firstCaseNoDefault, "foo case");

let firstCase;
switch ("foo") {
    case "foo":
        firstCase = "foo case";
        break;
    case "bar":
        firstCase = "bar case";
        break;
    default:
        firstCase = "default case";
        break;
}
console.assertEqual(firstCase, "foo case");

let secondCase;
switch ("bar") {
    case "foo":
        secondCase = "foo case";
        break;
    case "bar":
        secondCase = "bar case";
        break;
    default:
        secondCase = "default case";
        break;
}
console.assertEqual(secondCase, "bar case");

let defaultCase;
switch ("baz") {
    case "foo":
        defaultCase = "foo case";
        break;
    case "bar":
        defaultCase = "bar case";
        break;
    default:
        defaultCase = "default case";
        break;
}
console.assertEqual(defaultCase, "default case");

switch ("baz") {
    case "foo":
        console.assertNotReached();
        break;
    case "bar":
        console.assertNotReached();
        break;
}

let fallthrough;
switch ("foo") {
    case "foo":
        fallthrough = "foo case";
    case "bar":
        fallthrough = "bar case";
        break;
    case "baz":
        fallthrough = "baz case";
        break;
    default:
        fallthrough = "default case";
        break;
}
console.assertEqual(fallthrough, "bar case");

let fallthroughToDefault;
switch ("foo") {
    case "foo":
        fallthroughToDefault = "foo case";
    case "bar":
        fallthroughToDefault = "bar case";
    default:
        fallthroughToDefault = "default case";
}
console.assertEqual(fallthroughToDefault, "default case");

let called1, called2, called3;
function f1(n) {
    called1 = true;
    return n;
}
function f2(n) {
    called2 = true;
    return n;
}
function f3(n) {
    called3 = true;
    return n;
}
let nonLiteralCase;
switch (100 + 32) {
    case f1(131):
        nonLiteralCase = "f1 case";
        break;
    case f2(132):
        nonLiteralCase = "f2 case";
        break;
    case f3(133):
        nonLiteralCase = "f3 case";
        break;
    default:
        nonLiteralCase = "default case";
        break;
}
console.assertEqual(nonLiteralCase, "f2 case");
console.assert(called1);
console.assert(called2);
console.assert(!called3);

// FIXME: Switch statement with non-strict equality.
//let nonStrictEqual;
//switch (120 + 3) {
//    case "12" + "3":
//        nonStrictEqual = true;
//        break;
//}
//console.assert(nonStrictEqual);

function returnInCase() {
    switch ("foo") {
        case "foo":
            return "foo case";
        default:
            console.assertNotReached();
            break;
    }
    console.assertNotReached();
}
console.assertEqual(returnInCase(), "foo case");

function returnInDefaultCase() {
    switch ("bar") {
        case "foo":
            console.assertNotReached();
            break;
        default:
            return "default case";
    }
    console.assertNotReached();
}
console.assertEqual(returnInDefaultCase(), "default case");

let continueInCase;
while (!continueInCase) {
    switch ("foo") {
        case "foo":
            continueInCase = "foo case";
            continue;
        default:
            console.assertNotReached();
            break;
    }
    console.assertNotReached();
}
console.assertEqual(continueInCase, "foo case");

let continueInDefaultCase;
while (!continueInDefaultCase) {
    switch ("bar") {
        case "foo":
            console.assertNotReached();
            break;
        default:
            continueInDefaultCase = "default case";
            continue;
    }
    console.assertNotReached();
}
console.assertEqual(continueInDefaultCase, "default case");

let breakInCase;
while (!breakInCase) {
    switch ("foo") {
        case "foo":
            break;
        default:
            console.assertNotReached();
            break;
    }
    breakInCase = "break from switch not from loop";
}
console.assertEqual(breakInCase, "break from switch not from loop");

let breakInDefaultCase;
while (!breakInDefaultCase) {
    switch ("bar") {
        case "foo":
            console.assertNotReached();
            break;
        default:
            break;
    }
    breakInDefaultCase = "break from switch not from loop";
}
console.assertEqual(breakInDefaultCase, "break from switch not from loop");
