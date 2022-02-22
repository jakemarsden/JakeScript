var sales = 'Toyota';

function carTypes(name) {
    if (name === 'Honda') {
        return name;
    } else {
        return "Sorry, we don't sell " + name + ".";
    }
}

var car = { myCar: 'Saturn', getCar: carTypes('Honda'), special: sales };

assert car.myCar === "Saturn";   // Saturn
assert car.getCar === "Honda";  // Honda
assert car.special === "Toyota"; // Toyota
