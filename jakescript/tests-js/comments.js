let a = /* Comment 1 */ 10;
let b = 20; // Comment 2

let c /*
Comment 3
  * /
*
/
/
*/
= 30;

let /**Comment 4*/ d = /* 50 */ 40;

assert a + b + c + d === 100;
