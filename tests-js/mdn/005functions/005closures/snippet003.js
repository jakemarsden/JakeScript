var getCode = (function() {
  var apiCode = '0]Eal(eh&2';    // A code we do not want outsiders to be able to modify...

  return function() {
    return apiCode;
  };
})();

var result = getCode();    // Returns the apiCode
assert result === "0]Eal(eh&2";
