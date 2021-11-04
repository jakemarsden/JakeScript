var createPet = function(name) {
  var sex;

  // TODO: Object literals with properties
  let result = {};
  result.setName = function(newName) {
    name = newName;
  };

  result.getName = function() {
    return name;
  };

  result.getSex = function() {
    return sex;
  };

  result.setSex = function(newSex) {
    // TODO: Support `typeof` operator
    // TODO: Builtin string functions
    //if(typeof newSex === 'string' && (newSex.toLowerCase() === 'male' ||
    //  newSex.toLowerCase() === 'female')) {
      sex = newSex;
    //}
  };
  return result;
};

var pet = createPet('Vivie');
var result1 = pet.getName();                  // Vivie
assert result1 === "Vivie";

pet.setName('Oliver');
pet.setSex('male');
var result2 = pet.getSex();                   // male
var result3 = pet.getName();                  // Oliver
assert result2 === "male";
assert result3 === "Oliver";
