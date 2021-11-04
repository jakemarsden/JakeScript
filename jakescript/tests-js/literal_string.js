// In variable names:
// - S => Single quote
// - D => Double quote

let happyPathS = 'abc';
let happyPathD = "abc";
let emptyS = '';
let emptyD = "";
let dInsideS = '"';
let sInsideD = "'";

let escapedEscapeS = '\\';
let escapedEscapeD = "\\";
let escapedDelimiterS1 = '\'';
let escapedDelimiterD1 = "\"";
let escapedDelimiterS2 = '\'foo\'bar\'';
let escapedDelimiterD2 = "\"foo\"bar\"";
let escapedDInsideS1 = '\"';
let escapedSInsideD1 = "\'";
let escapedDInsideS2 = '\"foo\"bar\"';
let escapedSInsideD2 = "\'foo\'bar\'";
let escapeS = '\0\b\t\f\v\n\r\"\'\\';
let escapeD = "\0\b\t\f\v\n\r\"\'\\";

let lineContinuationS1 = 'Hello, \
world!';
let lineContinuationD1 = "Hello, \
world!";
let lineContinuationS2 = 'Hello, \\\
world!';
let lineContinuationD2 = "Hello, \\\
world!";
