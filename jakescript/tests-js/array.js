let empty = [];
console.assertEqual(empty.length, 0);

let two = 2;
let numbers = [1, two, "foo", null, "bar"];

console.assertEqual(numbers.length, 5);
console.assertEqual(numbers[0], 1);
console.assertEqual(numbers[1], 2);
console.assertEqual(numbers[2], "foo");
console.assertEqual(numbers[3], null);
console.assertEqual(numbers[4], "bar");
console.assertEqual(numbers[5], undefined);
console.assertEqual(numbers[500], undefined);

function square(n) {
    return n * n;
}

console.assertEqual(numbers[two], "foo");
console.assertEqual(numbers[1 + 2], null);
console.assertEqual(numbers[square(2)], "bar");
console.assertEqual(numbers[500 + 1], undefined);
