var log = "";

function foo(i) {
    // TODO: Braceless single-line if and loop statements
    if (i < 0) {
        return;
    }
    log += 'begin: ' + i + '\n';
    foo(i - 1);
    log += 'end: ' + i + '\n';
}
foo(3);

// Output:

// begin: 3
// begin: 2
// begin: 1
// begin: 0
// end: 0
// end: 1
// end: 2
// end: 3
assert log === "begin: 3\nbegin: 2\nbegin: 1\nbegin: 0\nend: 0\nend: 1\nend: 2\nend: 3\n";
