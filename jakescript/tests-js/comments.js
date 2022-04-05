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

/**
 * Comment ✔ with ∴ unicode ❌.
 */
let /**Comment 4*/ d = /* 50 */ 40;

// Comment ✔ with ∴ unicode ❌.
console.assertEqual(a + b + c + d /**//***//****//*****/,/**/ 100);
