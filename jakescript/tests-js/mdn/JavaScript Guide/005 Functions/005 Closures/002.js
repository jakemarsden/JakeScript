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
      //  newSex.toLowerCase() === 'female')) {
        sex = newSex;
      //}
    }
  };
};

var pet = createPet('Vivie');
pet.getName();                  // Vivie

console.assert(pet.getName() === "Vivie");

pet.setName('Oliver');
pet.setSex('male');
pet.getSex();                   // male
pet.getName();                  // Oliver

console.assert(pet.getSex() === "male");
console.assert(pet.getName() === "Oliver");
