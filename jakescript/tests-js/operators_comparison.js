console.assert(0 == 0);
console.assert(!(0 == 5));
console.assert(!(0 == -5));
console.assert(!(0 == Infinity));
console.assert(!(0 == -Infinity));
console.assert(!(0 == NaN));

console.assert(!(5 == 0));
console.assert(5 == 5);
console.assert(!(5 == -5));
console.assert(!(5 == Infinity));
console.assert(!(5 == -Infinity));
console.assert(!(5 == NaN));

console.assert(!(-5 == 0));
console.assert(!(-5 == 5));
console.assert(-5 == -5);
console.assert(!(-5 == Infinity));
console.assert(!(-5 == -Infinity));
console.assert(!(-5 == NaN));

console.assert(!(Infinity == 0));
console.assert(!(Infinity == 5));
console.assert(!(Infinity == -5));
console.assert(Infinity == Infinity);
console.assert(!(Infinity == -Infinity));
console.assert(!(Infinity == NaN));

console.assert(!(-Infinity == 0));
console.assert(!(-Infinity == 5));
console.assert(!(-Infinity == -5));
console.assert(!(-Infinity == Infinity));
console.assert(-Infinity == -Infinity);
console.assert(!(-Infinity == NaN));

console.assert(!(NaN == 0));
console.assert(!(NaN == 5));
console.assert(!(NaN == -5));
console.assert(!(NaN == Infinity));
console.assert(!(NaN == -Infinity));
console.assert(!(NaN == NaN));

console.assert(!(0 != 0));
console.assert(0 != 5);
console.assert(0 != -5);
console.assert(0 != Infinity);
console.assert(0 != -Infinity);
console.assert(0 != NaN);

console.assert(5 != 0);
console.assert(!(5 != 5));
console.assert(5 != -5);
console.assert(5 != Infinity);
console.assert(5 != -Infinity);
console.assert(5 != NaN);

console.assert(-5 != 0);
console.assert(-5 != 5);
console.assert(!(-5 != -5));
console.assert(-5 != Infinity);
console.assert(-5 != -Infinity);
console.assert(-5 != NaN);

console.assert(Infinity != 0);
console.assert(Infinity != 5);
console.assert(Infinity != -5);
console.assert(!(Infinity != Infinity));
console.assert(Infinity != -Infinity);
console.assert(Infinity != NaN);

console.assert(-Infinity != 0);
console.assert(-Infinity != 5);
console.assert(-Infinity != -5);
console.assert(-Infinity != Infinity);
console.assert(!(-Infinity != -Infinity));
console.assert(-Infinity != NaN);

console.assert(NaN != 0);
console.assert(NaN != 5);
console.assert(NaN != -5);
console.assert(NaN != Infinity);
console.assert(NaN != -Infinity);
console.assert(NaN != NaN);

console.assert(0 === 0);
console.assert(!(0 === 5));
console.assert(!(0 === -5));
console.assert(!(0 === Infinity));
console.assert(!(0 === -Infinity));
console.assert(!(0 === NaN));

console.assert(!(5 === 0));
console.assert(5 === 5);
console.assert(!(5 === -5));
console.assert(!(5 === Infinity));
console.assert(!(5 === -Infinity));
console.assert(!(5 === NaN));

console.assert(!(-5 === 0));
console.assert(!(-5 === 5));
console.assert(-5 === -5);
console.assert(!(-5 === Infinity));
console.assert(!(-5 === -Infinity));
console.assert(!(-5 === NaN));

console.assert(!(Infinity === 0));
console.assert(!(Infinity === 5));
console.assert(!(Infinity === -5));
console.assert(Infinity === Infinity);
console.assert(!(Infinity === -Infinity));
console.assert(!(Infinity === NaN));

console.assert(!(-Infinity === 0));
console.assert(!(-Infinity === 5));
console.assert(!(-Infinity === -5));
console.assert(!(-Infinity === Infinity));
console.assert(-Infinity === -Infinity);
console.assert(!(-Infinity === NaN));

console.assert(!(NaN === 0));
console.assert(!(NaN === 5));
console.assert(!(NaN === -5));
console.assert(!(NaN === Infinity));
console.assert(!(NaN === -Infinity));
console.assert(!(NaN === NaN));

console.assert(!(0 !== 0));
console.assert(0 !== 5);
console.assert(0 !== -5);
console.assert(0 !== Infinity);
console.assert(0 !== -Infinity);
console.assert(0 !== NaN);

console.assert(5 !== 0);
console.assert(!(5 !== 5));
console.assert(5 !== -5);
console.assert(5 !== Infinity);
console.assert(5 !== -Infinity);
console.assert(5 !== NaN);

console.assert(-5 !== 0);
console.assert(-5 !== 5);
console.assert(!(-5 !== -5));
console.assert(-5 !== Infinity);
console.assert(-5 !== -Infinity);
console.assert(-5 !== NaN);

console.assert(Infinity !== 0);
console.assert(Infinity !== 5);
console.assert(Infinity !== -5);
console.assert(!(Infinity !== Infinity));
console.assert(Infinity !== -Infinity);
console.assert(Infinity !== NaN);

console.assert(-Infinity !== 0);
console.assert(-Infinity !== 5);
console.assert(-Infinity !== -5);
console.assert(-Infinity !== Infinity);
console.assert(!(-Infinity !== -Infinity));
console.assert(-Infinity !== NaN);

console.assert(NaN !== 0);
console.assert(NaN !== 5);
console.assert(NaN !== -5);
console.assert(NaN !== Infinity);
console.assert(NaN !== -Infinity);
console.assert(NaN !== NaN);

console.assert(!(0 < 0));
console.assert(0 < 5);
console.assert(!(0 < -5));
console.assert(0 < Infinity);
console.assert(!(0 < -Infinity));
console.assert(!(0 < NaN));

console.assert(!(5 < 0));
console.assert(!(5 < 5));
console.assert(!(5 < -5));
console.assert(5 < Infinity);
console.assert(!(5 < -Infinity));
console.assert(!(5 < NaN));

console.assert(-5 < 0);
console.assert(-5 < 5);
console.assert(!(-5 < -5));
console.assert(-5 < Infinity);
console.assert(!(-5 < -Infinity));
console.assert(!(-5 < NaN));

console.assert(!(Infinity < 0));
console.assert(!(Infinity < 5));
console.assert(!(Infinity < -5));
console.assert(!(Infinity < Infinity));
console.assert(!(Infinity < -Infinity));
console.assert(!(Infinity < NaN));

console.assert(-Infinity < 0);
console.assert(-Infinity < 5);
console.assert(-Infinity < -5);
console.assert(-Infinity < Infinity);
console.assert(!(-Infinity < -Infinity));
console.assert(!(-Infinity < NaN));

console.assert(!(NaN < 0));
console.assert(!(NaN < 5));
console.assert(!(NaN < -5));
console.assert(!(NaN < Infinity));
console.assert(!(NaN < -Infinity));
console.assert(!(NaN < NaN));

console.assert(0 <= 0);
console.assert(0 <= 5);
console.assert(!(0 <= -5));
console.assert(0 <= Infinity);
console.assert(!(0 <= -Infinity));
console.assert(!(0 <= NaN));

console.assert(!(5 <= 0));
console.assert(5 <= 5);
console.assert(!(5 <= -5));
console.assert(5 <= Infinity);
console.assert(!(5 <= -Infinity));
console.assert(!(5 <= NaN));

console.assert(-5 <= 0);
console.assert(-5 <= 5);
console.assert(-5 <= -5);
console.assert(-5 <= Infinity);
console.assert(!(-5 <= -Infinity));
console.assert(!(-5 <= NaN));

console.assert(!(Infinity <= 0));
console.assert(!(Infinity <= 5));
console.assert(!(Infinity <= -5));
console.assert(Infinity <= Infinity);
console.assert(!(Infinity <= -Infinity));
console.assert(!(Infinity <= NaN));

console.assert(-Infinity <= 0);
console.assert(-Infinity <= 5);
console.assert(-Infinity <= -5);
console.assert(-Infinity <= Infinity);
console.assert(-Infinity <= -Infinity);
console.assert(!(-Infinity <= NaN));

console.assert(!(NaN <= 0));
console.assert(!(NaN <= 5));
console.assert(!(NaN <= -5));
console.assert(!(NaN <= Infinity));
console.assert(!(NaN <= -Infinity));
console.assert(!(NaN <= NaN));

console.assert(!(0 > 0));
console.assert(!(0 > 5));
console.assert(0 > -5);
console.assert(!(0 > Infinity));
console.assert(0 > -Infinity);
console.assert(!(0 > NaN));

console.assert(5 > 0);
console.assert(!(5 > 5));
console.assert(5 > -5);
console.assert(!(5 > Infinity));
console.assert(5 > -Infinity);
console.assert(!(5 > NaN));

console.assert(!(-5 > 0));
console.assert(!(-5 > 5));
console.assert(!(-5 > -5));
console.assert(!(-5 > Infinity));
console.assert(-5 > -Infinity);
console.assert(!(-5 > NaN));

console.assert(Infinity > 0);
console.assert(Infinity > 5);
console.assert(Infinity > -5);
console.assert(!(Infinity > Infinity));
console.assert(Infinity > -Infinity);
console.assert(!(Infinity > NaN));

console.assert(!(-Infinity > 0));
console.assert(!(-Infinity > 5));
console.assert(!(-Infinity > -5));
console.assert(!(-Infinity > Infinity));
console.assert(!(-Infinity > -Infinity));
console.assert(!(-Infinity > NaN));

console.assert(!(NaN > 0));
console.assert(!(NaN > 5));
console.assert(!(NaN > -5));
console.assert(!(NaN > Infinity));
console.assert(!(NaN > -Infinity));
console.assert(!(NaN > NaN));

console.assert(0 >= 0);
console.assert(!(0 >= 5));
console.assert(0 >= -5);
console.assert(!(0 >= Infinity));
console.assert(0 >= -Infinity);
console.assert(!(0 >= NaN));

console.assert(5 >= 0);
console.assert(5 >= 5);
console.assert(5 >= -5);
console.assert(!(5 >= Infinity));
console.assert(5 >= -Infinity);
console.assert(!(5 >= NaN));

console.assert(!(-5 >= 0));
console.assert(!(-5 >= 5));
console.assert(-5 >= -5);
console.assert(!(-5 >= Infinity));
console.assert(-5 >= -Infinity);
console.assert(!(-5 >= NaN));

console.assert(Infinity >= 0);
console.assert(Infinity >= 5);
console.assert(Infinity >= -5);
console.assert(Infinity >= Infinity);
console.assert(Infinity >= -Infinity);
console.assert(!(Infinity >= NaN));

console.assert(!(-Infinity >= 0));
console.assert(!(-Infinity >= 5));
console.assert(!(-Infinity >= -5));
console.assert(!(-Infinity >= Infinity));
console.assert(-Infinity >= -Infinity);
console.assert(!(-Infinity >= NaN));

console.assert(!(NaN >= 0));
console.assert(!(NaN >= 5));
console.assert(!(NaN >= -5));
console.assert(!(NaN >= Infinity));
console.assert(!(NaN >= -Infinity));
console.assert(!(NaN >= NaN));

let counter = 0;
function assertFirst(n) {
    console.assert(counter % 2 === 0);
    counter += 1;
    return n;
}
function assertSecond(n) {
    console.assert(counter % 2 === 1);
    counter += 1;
    return n;
}

console.assert(assertFirst(3) == assertSecond(3));
console.assert(assertFirst(3) != assertSecond(5));
console.assert(assertFirst(3) === assertSecond(3));
console.assert(assertFirst(3) !== assertSecond(5));
console.assert(assertFirst(3) < assertSecond(5));
console.assert(assertFirst(3) <= assertSecond(5));
console.assert(assertFirst(5) > assertSecond(3));
console.assert(assertFirst(5) >= assertSecond(3));
console.assert(counter === 16);
