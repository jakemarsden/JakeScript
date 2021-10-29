function myFunc(theObject) {
  theObject.make = 'Toyota';
}

// TODO: Object literals with properties
var mycar = {};
mycar.make = 'Honda';
mycar.model = 'Accord';
mycar.year = 1998;
// TODO: Declare multiple variables in one statement
var x;
var y;

x = mycar.make; // x gets the value "Honda"
assert x === "Honda";

myFunc(mycar);
y = mycar.make; // y gets the value "Toyota"
                // (the make property was changed by the function)
assert y === "Toyota";
