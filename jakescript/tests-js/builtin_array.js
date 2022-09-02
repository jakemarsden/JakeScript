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

let shove = [1, 2];
console.assertEqual(shove.push(3), 3);
console.assertEqual(shove.length, 3);
console.assertEqual(shove[2], 3);

console.assertEqual(shove.push("foo", "bar"), 5);
console.assertEqual(shove.length, 5);
console.assertEqual(shove[3], "foo");
console.assertEqual(shove[4], "bar");

console.assertEqual(shove.push(), 5);
console.assertEqual(shove.length, 5);
console.assertEqual(shove[0], 1);
console.assertEqual(shove[1], 2);
console.assertEqual(shove[2], 3);
console.assertEqual(shove[3], "foo");
console.assertEqual(shove[4], "bar");

let emptyShove = [];
console.assertEqual(emptyShove.push(1, 2), 2);
console.assertEqual(emptyShove.length, 2);
console.assertEqual(emptyShove[0], 1);
console.assertEqual(emptyShove[1], 2);

let shoveInnerArray = [];
console.assertEqual(shoveInnerArray.push([1, 2, 3]), 1);
console.assertEqual(shoveInnerArray.length, 1);
console.assertEqual(shoveInnerArray[0].length, 3);
console.assertEqual(shoveInnerArray[0][0], 1);
console.assertEqual(shoveInnerArray[0][1], 2);
console.assertEqual(shoveInnerArray[0][2], 3);
