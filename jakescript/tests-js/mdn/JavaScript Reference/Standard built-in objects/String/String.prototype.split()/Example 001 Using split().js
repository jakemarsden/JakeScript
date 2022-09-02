const emptyString = '';

// string is empty and no separator is specified
console.log(emptyString.split());
// [""]

// string and separator are both empty strings
console.log(emptyString.split(emptyString));
// []
