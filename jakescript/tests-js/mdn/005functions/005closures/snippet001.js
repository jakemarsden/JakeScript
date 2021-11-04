var pet = function(name) {   // The outer function defines a variable called "name"
  var getName = function() {
    return name;             // The inner function has access to the "name" variable of the outer
                             //function
  };
  return getName;            // Return the inner function, thereby exposing it to outer scopes
};
var myPet = pet('Vivie');

var result = myPet();                     // Returns "Vivie"
assert result === "Vivie";
