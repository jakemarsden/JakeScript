var mockLog = "";
var parentLogger = console.log;
console.log = function (msg) {
  mockLog += msg + '\n';
  parentLogger(msg);
};

function foo(i) {
  if (i < 0)
    return;
  console.log('begin: ' + i);
  foo(i - 1);
  console.log('end: ' + i);
}
foo(3);

// Output:

// begin: 3
// begin: 2
// begin: 1
// begin: 0
// end: 0
// end: 1
// end: 2
// end: 3

console.assert(mockLog === "begin: 3\nbegin: 2\nbegin: 1\nbegin: 0\nend: 0\nend: 1\nend: 2\nend: 3\n");
