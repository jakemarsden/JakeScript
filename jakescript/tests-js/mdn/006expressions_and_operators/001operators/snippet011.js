var mockLog = "";
console.log = function(msg) {
  mockLog += msg + '\n';
};

console.log('my ' + 'string'); // console logs the string "my string".
console.assert(mockLog === "my string\n");
