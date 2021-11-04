let happyPath = /abc/g;
let noFlags = /abc/;
let allTheFlags = /abc/dgimsuy;

// The spec suggests using `/(?:)/` as a workaround for specifying empty regexes, since `//` is
// parsed as the start of a single-line comment.
let fakeEmpty = /(?:)/;

let escapedEscape = /\\/;
let escapedDelimiter1 = /\//;
let escapedDelimiter2 = /\/foo\/bar\//;
let escapedClassStart = /\[abc/;
let closeBracketOutsideClass = /]/;

let characterClasses = /foob[ar]{2}b[az]{2}/;
let nestedClassesArentAThing1 = /[[]/;
let nestedClassesArentAThing2 = /[[[[[]/;
let escapeInsideClass1 = /[\\]/;
let escapeInsideClass2 = /[\]]/;
let escapeInsideClass3 = /[\n]/;
let delimiterInsideClass = /[/]/;

// A valid `RegularExpressionChar` but not a valid `RegularExpressionFirstChar`.
let repeating1 = /a*/;
let repeating2 = /\*/;

// From: https://www.regular-expressions.info/email.html
let email1 = /^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$/g;
let email2 = /\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b/ig;
let scaryEmail = /\A(?:[a-z0-9!#$%&'*+/=?^_‘{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_‘{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])\z/g;
