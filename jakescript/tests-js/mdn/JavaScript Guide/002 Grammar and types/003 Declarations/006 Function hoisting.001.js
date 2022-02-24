var mockLog = "";
var oldLogger = console.log;
console.log = function (msg) {
  mockLog += msg + '\n';
  oldLogger(msg);
};

/* Function declaration */

foo(); // "bar"

function foo() {
  console.log('bar');
}

/* Function expression */

//baz(); // TypeError: baz is not a function

var baz = function() {
  console.log('bar2');
};

console.assert(mockLog === "bar\n");
