var myFunctionCalled = false;
function myFunction() {
  myFunctionCalled = true;
}

var myArray = [];
if (!myArray[0]) myFunction();

console.assert(myFunctionCalled);
