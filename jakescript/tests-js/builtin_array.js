console.assert(Array);

let three = Array(2, 4, 6);
console.assertEqual(three[0], 2);
console.assertEqual(three[1], 4);
console.assertEqual(three[2], 6);
console.assertEqual(three[3], undefined);

let two = Array(2, 4);
console.assertEqual(two[0], 2);
console.assertEqual(two[1], 4);
console.assertEqual(two[2], undefined);

let one = Array(2);
console.assertEqual(one[0], 2);
console.assertEqual(one[1], undefined);

let empty = Array();
console.assertEqual(empty[0], undefined);

let undef = Array(undefined);
console.assertEqual(undef[0], undefined);
console.assertEqual(undef[1], undefined);

console.assertEqual(Array(2, 4, 6).length, 3);
console.assertEqual(Array(2, 4).length, 2);
console.assertEqual(Array(2).length, 1);
console.assertEqual(Array().length, 0);
console.assertEqual(Array(undefined).length, 1);

let foo = Array("foo", "bar", "baz");
console.assertEqual(foo.length, 3);
foo.length = 123;
console.assertEqual(foo.length, 3);
