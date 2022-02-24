var num = 0;

var myFunc;
if (num === 0) {
  myFunc = function(theObject) {
    theObject.make = 'Toyota';
  };
}

console.assert(myFunc);
