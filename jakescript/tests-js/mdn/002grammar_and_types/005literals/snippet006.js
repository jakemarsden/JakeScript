var sales = 'Toyota';

function carTypes(name) {
    if (name === 'Honda') {
        return name;
    } else {
        return "Sorry, we don't sell " + name + ".";
    }
}

// TODO: Object literals with properties
var car = {};
car.myCar = 'Saturn';
car.getCar = carTypes('Honda');
car.special = sales;

assert car.myCar === "Saturn";   // Saturn
assert car.getCar === "Honda";  // Honda
assert car.special === "Toyota"; // Toyota
