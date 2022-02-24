var mockLog = "";
var parentLogger = console.log;
console.log = function (msg) {
  mockLog += msg + "\n";
  parentLogger(msg);
};

var quote = "He read \"The Cremation of Sam McGee\" by R.W. Service.";
console.log(quote);

console.assert(mockLog === 'He read "The Cremation of Sam McGee" by R.W. Service.\n');
