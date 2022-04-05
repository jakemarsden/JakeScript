let greetWorld = function() {
    return "Hello, world!";
};
console.assertEqual(greetWorld(), "Hello, world!");

let greet = function(recipient) {
    return "Hello, " + recipient + "!";
};
console.assertEqual(greet("Jake"), "Hello, Jake!");

let greet2 = function(greeting, recipient) {
    return greeting + ", " + recipient + "!";
};
console.assertEqual(greet2("Hullo", "old chap"), "Hullo, old chap!");

let higherOrderGreet = function(greetingProvider, recipientProvider) {
    const greeting = greetingProvider();
    const recipient = recipientProvider();
    return greet2(greeting, recipient);
};
console.assertEqual(higherOrderGreet(function () { return "Hullo"; }, function () { return "world"; }), "Hullo, world!");

let meaning = (function() {
    return 42;
})();
console.assertEqual(meaning, 42);

let squared = (function(number) {
    return number * number;
})(5);
console.assertEqual(squared, 25);
