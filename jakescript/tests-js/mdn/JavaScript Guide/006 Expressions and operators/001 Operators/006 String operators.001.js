var mockLog = "";
var parentLogger = console.log;
console.log = function (msg) {
  mockLog += msg + '\n';
  parentLogger(msg);
};

console.log('my ' + 'string'); // console logs the string "my string".

console.assert(mockLog === "my string\n");
