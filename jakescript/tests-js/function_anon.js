let greetWorld = function() {
    return "Hello, world!";
};
console.assert(greetWorld() === "Hello, world!");

let greet = function(recipient) {
    return "Hello, " + recipient + "!";
};
console.assert(greet("Jake") === "Hello, Jake!");

let greet2 = function(greeting, recipient) {
    return greeting + ", " + recipient + "!";
};
console.assert(greet2("Hullo", "old chap") === "Hullo, old chap!");

let higherOrderGreet = function(greetingProvider, recipientProvider) {
    const greeting = greetingProvider();
    const recipient = recipientProvider();
    return greet2(greeting, recipient);
};
console.assert(higherOrderGreet(function () { return "Hullo"; }, function () { return "world"; }) === "Hullo, world!");

let meaning = (function() {
    return 42;
})();
console.assert(meaning === 42);

let squared = (function(number) {
    return number * number;
})(5);
console.assert(squared === 25);
