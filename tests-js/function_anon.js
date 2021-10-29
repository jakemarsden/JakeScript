let greetWorld = function() {
    return "Hello, world!";
};
assert greetWorld() === "Hello, world!";

let greet = function(recipient) {
    return "Hello, " + recipient + "!";
};
assert greet("Jake") === "Hello, Jake!";

let greet2 = function(greeting, recipient) {
    return greeting + ", " + recipient + "!";
};
assert greet2("Hullo", "old chap") === "Hullo, old chap!";

let higherOrderGreet = function(greetingProvider, recipientProvider) {
    const greeting = greetingProvider();
    const recipient = recipientProvider();
    return greet2(greeting, recipient);
};
assert higherOrderGreet(function () { return "Hullo"; }, function () { return "world"; }) === "Hullo, world!";

let meaning = (function() {
    return 42;
})();
assert meaning === 42;

let squared = (function(number) {
    return number * number;
})(5);
assert squared === 25;
