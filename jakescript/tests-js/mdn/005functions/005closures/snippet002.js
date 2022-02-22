var createPet = function(name) {
  var sex;

  return {
    setName: function(newName) {
      name = newName;
    },

    getName: function() {
      return name;
    },

    getSex: function() {
      return sex;
    },

    setSex: function(newSex) {
      // TODO: Support `typeof` operator
      // TODO: Builtin string functions
      //if(typeof newSex === 'string' && (newSex.toLowerCase() === 'male' ||
      //    newSex.toLowerCase() === 'female')) {
        sex = newSex;
      //}
    }
  };
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
