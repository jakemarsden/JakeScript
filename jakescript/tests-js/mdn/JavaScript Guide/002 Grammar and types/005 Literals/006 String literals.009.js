var mockLog = "";
var parentLogger = console.log;
console.log = function (msg) {
  mockLog += msg + "\n";
  parentLogger(msg);
};

var str = 'this string \
is broken \
across multiple \
lines.';
console.log(str);   // this string is broken across multiple lines.

console.assert(mockLog === "this string is broken across multiple lines.\n");
