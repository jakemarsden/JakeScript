assert 0 == 0;
assert !(0 == 5);
assert !(0 == -5);
assert !(0 == Infinity);
assert !(0 == -Infinity);
assert !(0 == NaN);

assert !(5 == 0);
assert 5 == 5;
assert !(5 == -5);
assert !(5 == Infinity);
assert !(5 == -Infinity);
assert !(5 == NaN);

assert !(-5 == 0);
assert !(-5 == 5);
assert -5 == -5;
assert !(-5 == Infinity);
assert !(-5 == -Infinity);
assert !(-5 == NaN);

assert !(Infinity == 0);
assert !(Infinity == 5);
assert !(Infinity == -5);
assert Infinity == Infinity;
assert !(Infinity == -Infinity);
assert !(Infinity == NaN);

assert !(-Infinity == 0);
assert !(-Infinity == 5);
assert !(-Infinity == -5);
assert !(-Infinity == Infinity);
assert -Infinity == -Infinity;
assert !(-Infinity == NaN);

assert !(NaN == 0);
assert !(NaN == 5);
assert !(NaN == -5);
assert !(NaN == Infinity);
assert !(NaN == -Infinity);
assert !(NaN == NaN);

assert !(0 != 0);
assert 0 != 5;
assert 0 != -5;
assert 0 != Infinity;
assert 0 != -Infinity;
assert 0 != NaN;

assert 5 != 0;
assert !(5 != 5);
assert 5 != -5;
assert 5 != Infinity;
assert 5 != -Infinity;
assert 5 != NaN;

assert -5 != 0;
assert -5 != 5;
assert !(-5 != -5);
assert -5 != Infinity;
assert -5 != -Infinity;
assert -5 != NaN;

assert Infinity != 0;
assert Infinity != 5;
assert Infinity != -5;
assert !(Infinity != Infinity);
assert Infinity != -Infinity;
assert Infinity != NaN;

assert -Infinity != 0;
assert -Infinity != 5;
assert -Infinity != -5;
assert -Infinity != Infinity;
assert !(-Infinity != -Infinity);
assert -Infinity != NaN;

assert NaN != 0;
assert NaN != 5;
assert NaN != -5;
assert NaN != Infinity;
assert NaN != -Infinity;
assert NaN != NaN;

assert 0 === 0;
assert !(0 === 5);
assert !(0 === -5);
assert !(0 === Infinity);
assert !(0 === -Infinity);
assert !(0 === NaN);

assert !(5 === 0);
assert 5 === 5;
assert !(5 === -5);
assert !(5 === Infinity);
assert !(5 === -Infinity);
assert !(5 === NaN);

assert !(-5 === 0);
assert !(-5 === 5);
assert -5 === -5;
assert !(-5 === Infinity);
assert !(-5 === -Infinity);
assert !(-5 === NaN);

assert !(Infinity === 0);
assert !(Infinity === 5);
assert !(Infinity === -5);
assert Infinity === Infinity;
assert !(Infinity === -Infinity);
assert !(Infinity === NaN);

assert !(-Infinity === 0);
assert !(-Infinity === 5);
assert !(-Infinity === -5);
assert !(-Infinity === Infinity);
assert -Infinity === -Infinity;
assert !(-Infinity === NaN);

assert !(NaN === 0);
assert !(NaN === 5);
assert !(NaN === -5);
assert !(NaN === Infinity);
assert !(NaN === -Infinity);
assert !(NaN === NaN);

assert !(0 !== 0);
assert 0 !== 5;
assert 0 !== -5;
assert 0 !== Infinity;
assert 0 !== -Infinity;
assert 0 !== NaN;

assert 5 !== 0;
assert !(5 !== 5);
assert 5 !== -5;
assert 5 !== Infinity;
assert 5 !== -Infinity;
assert 5 !== NaN;

assert -5 !== 0;
assert -5 !== 5;
assert !(-5 !== -5);
assert -5 !== Infinity;
assert -5 !== -Infinity;
assert -5 !== NaN;

assert Infinity !== 0;
assert Infinity !== 5;
assert Infinity !== -5;
assert !(Infinity !== Infinity);
assert Infinity !== -Infinity;
assert Infinity !== NaN;

assert -Infinity !== 0;
assert -Infinity !== 5;
assert -Infinity !== -5;
assert -Infinity !== Infinity;
assert !(-Infinity !== -Infinity);
assert -Infinity !== NaN;

assert NaN !== 0;
assert NaN !== 5;
assert NaN !== -5;
assert NaN !== Infinity;
assert NaN !== -Infinity;
assert NaN !== NaN;

assert !(0 < 0);
assert 0 < 5;
assert !(0 < -5);
assert 0 < Infinity;
assert !(0 < -Infinity);
assert !(0 < NaN);

assert !(5 < 0);
assert !(5 < 5);
assert !(5 < -5);
assert 5 < Infinity;
assert !(5 < -Infinity);
assert !(5 < NaN);

assert -5 < 0;
assert -5 < 5;
assert !(-5 < -5);
assert -5 < Infinity;
assert !(-5 < -Infinity);
assert !(-5 < NaN);

assert !(Infinity < 0);
assert !(Infinity < 5);
assert !(Infinity < -5);
assert !(Infinity < Infinity);
assert !(Infinity < -Infinity);
assert !(Infinity < NaN);

assert -Infinity < 0;
assert -Infinity < 5;
assert -Infinity < -5;
assert -Infinity < Infinity;
assert !(-Infinity < -Infinity);
assert !(-Infinity < NaN);

assert !(NaN < 0);
assert !(NaN < 5);
assert !(NaN < -5);
assert !(NaN < Infinity);
assert !(NaN < -Infinity);
assert !(NaN < NaN);

assert 0 <= 0;
assert 0 <= 5;
assert !(0 <= -5);
assert 0 <= Infinity;
assert !(0 <= -Infinity);
assert !(0 <= NaN);

assert !(5 <= 0);
assert 5 <= 5;
assert !(5 <= -5);
assert 5 <= Infinity;
assert !(5 <= -Infinity);
assert !(5 <= NaN);

assert -5 <= 0;
assert -5 <= 5;
assert -5 <= -5;
assert -5 <= Infinity;
assert !(-5 <= -Infinity);
assert !(-5 <= NaN);

assert !(Infinity <= 0);
assert !(Infinity <= 5);
assert !(Infinity <= -5);
assert Infinity <= Infinity;
assert !(Infinity <= -Infinity);
assert !(Infinity <= NaN);

assert -Infinity <= 0;
assert -Infinity <= 5;
assert -Infinity <= -5;
assert -Infinity <= Infinity;
assert -Infinity <= -Infinity;
assert !(-Infinity <= NaN);

assert !(NaN <= 0);
assert !(NaN <= 5);
assert !(NaN <= -5);
assert !(NaN <= Infinity);
assert !(NaN <= -Infinity);
assert !(NaN <= NaN);

assert !(0 > 0);
assert !(0 > 5);
assert 0 > -5;
assert !(0 > Infinity);
assert 0 > -Infinity;
assert !(0 > NaN);

assert 5 > 0;
assert !(5 > 5);
assert 5 > -5;
assert !(5 > Infinity);
assert 5 > -Infinity;
assert !(5 > NaN);

assert !(-5 > 0);
assert !(-5 > 5);
assert !(-5 > -5);
assert !(-5 > Infinity);
assert -5 > -Infinity;
assert !(-5 > NaN);

assert Infinity > 0;
assert Infinity > 5;
assert Infinity > -5;
assert !(Infinity > Infinity);
assert Infinity > -Infinity;
assert !(Infinity > NaN);

assert !(-Infinity > 0);
assert !(-Infinity > 5);
assert !(-Infinity > -5);
assert !(-Infinity > Infinity);
assert !(-Infinity > -Infinity);
assert !(-Infinity > NaN);

assert !(NaN > 0);
assert !(NaN > 5);
assert !(NaN > -5);
assert !(NaN > Infinity);
assert !(NaN > -Infinity);
assert !(NaN > NaN);

assert 0 >= 0;
assert !(0 >= 5);
assert 0 >= -5;
assert !(0 >= Infinity);
assert 0 >= -Infinity;
assert !(0 >= NaN);

assert 5 >= 0;
assert 5 >= 5;
assert 5 >= -5;
assert !(5 >= Infinity);
assert 5 >= -Infinity;
assert !(5 >= NaN);

assert !(-5 >= 0);
assert !(-5 >= 5);
assert -5 >= -5;
assert !(-5 >= Infinity);
assert -5 >= -Infinity;
assert !(-5 >= NaN);

assert Infinity >= 0;
assert Infinity >= 5;
assert Infinity >= -5;
assert Infinity >= Infinity;
assert Infinity >= -Infinity;
assert !(Infinity >= NaN);

assert !(-Infinity >= 0);
assert !(-Infinity >= 5);
assert !(-Infinity >= -5);
assert !(-Infinity >= Infinity);
assert -Infinity >= -Infinity;
assert !(-Infinity >= NaN);

assert !(NaN >= 0);
assert !(NaN >= 5);
assert !(NaN >= -5);
assert !(NaN >= Infinity);
assert !(NaN >= -Infinity);
assert !(NaN >= NaN);

let counter = 0;
function assertFirst(n) {
    assert counter % 2 === 0;
    counter += 1;
    return n;
}
function assertSecond(n) {
    assert counter % 2 === 1;
    counter += 1;
    return n;
}

assert assertFirst(3) == assertSecond(3);
assert assertFirst(3) != assertSecond(5);
assert assertFirst(3) === assertSecond(3);
assert assertFirst(3) !== assertSecond(5);
assert assertFirst(3) < assertSecond(5);
assert assertFirst(3) <= assertSecond(5);
assert assertFirst(5) > assertSecond(3);
assert assertFirst(5) >= assertSecond(3);
assert counter === 16;
