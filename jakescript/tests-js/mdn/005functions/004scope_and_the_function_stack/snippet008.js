let mockLog = "";
console.log = function (msg) {
  mockLog += msg + '\n';
};

function A(x) {
  function B(y) {
    function C(z) {
      console.log(x + y + z);
    }
    C(3);
  }
  B(2);
}
A(1); // logs 6 (1 + 2 + 3)

console.assert(mockLog === "6\n");
