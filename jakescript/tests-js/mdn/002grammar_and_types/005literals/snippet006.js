var sales = 'Toyota';

function carTypes(name) {
  if (name === 'Honda') {
    return name;
  } else {
    return "Sorry, we don't sell " + name + ".";
  }
}

var car = { myCar: 'Saturn', getCar: carTypes('Honda'), special: sales };

console.assert(car.myCar === "Saturn");
console.assert(car.getCar === "Honda");
console.assert(car.special === "Toyota");
