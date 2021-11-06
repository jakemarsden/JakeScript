function assertNotReached() {
    assert false;
}

assert 1 + 2 === 3;

exit;
assertNotReached();
