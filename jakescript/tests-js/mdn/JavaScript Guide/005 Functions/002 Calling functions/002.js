var mockLog = "";
var parentLogger = console.log;
console.log = function (msg) {
  mockLog += msg + '\n';
  parentLogger(msg);
};

console.log(square(5));
/* ... */
function square(n) { return n * n; }

console.assert(mockLog === "25\n");
