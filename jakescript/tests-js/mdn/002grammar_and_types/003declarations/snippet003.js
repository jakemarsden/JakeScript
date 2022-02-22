var myFunctionCalled = false;
function myFunction() {
  myFunctionCalled = true;
}

var myArray = [];
// TODO: Braceless single-line if and loop statements
if (!myArray[0]) { myFunction(); }

console.assert(myFunctionCalled);
