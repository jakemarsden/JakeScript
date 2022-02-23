var doThisCalled = false;
var doThatCalled = false;
function doThis() {
  doThisCalled = true;
}
function doThat() {
  doThatCalled = true;
}

var input;
if (input === undefined) {
  doThis();
} else {
  doThat();
}

console.assert(doThisCalled);
console.assert(!doThatCalled);
