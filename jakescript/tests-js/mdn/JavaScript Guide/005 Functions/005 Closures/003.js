var getCode = (function() {
  var apiCode = '0]Eal(eh&2';    // A code we do not want outsiders to be able to modify...

  return function() {
    return apiCode;
  };
})();

getCode();    // Returns the apiCode

console.assert(getCode() === "0]Eal(eh&2");
