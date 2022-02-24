var mockLog = "";
var parentLogger = console.log;
console.log = function (msg) {
  mockLog += msg + '\n';
  parentLogger(msg);
};

var n = null;
console.log(n * 32); // Will log 0 to the console

console.assert(mockLog === "0\n");
