// `this` at the top level of a script refers to the global object.
console.assert(this);
console.assertEqual(this.Boolean, Boolean);
console.assertEqual(this.Infinity, Infinity);
console.assertEqual(this.NaN, NaN);
console.assertEqual(this.Number, Number);
console.assertEqual(this.String, String);
console.assertEqual(this.undefined, undefined);

// `this` in a function called without `new` refers to the global object.
let globalObj = this;
function withoutNew() {
    console.assertEqual(this, globalObj);
}
withoutNew();

// `this` in a function called with an object as the receiver, refers to said object.
function createAnimal(species) {
    return {
        speak: function () {
            if (this.species === "cat") return "meow";
            if (this.species === "dog") return "woof";
            return undefined;
        },
        speakLots: function () {
            return this.speak() + " " + this.speak();
        },
        species: species,
    };
}
let cat = createAnimal("cat");
let dog = createAnimal("dog");
console.assertEqual(cat.speak(), "meow");
console.assertEqual(dog.speak(), "woof");
console.assertEqual(cat.speakLots(), "meow meow");
console.assertEqual(dog.speakLots(), "woof woof");

// `this` in a function called on an object refers to said object.
let animalObj = {
    speak: function () {
        if (this.species === "cat") return "meow";
        if (this.species === "dog") return "woof";
        return undefined;
    },
    speakLots: function () {
        return this.speak() + " " + this.speak();
    }
};
animalObj.species = "cat";
console.assertEqual(animalObj.speak(), "meow");
console.assertEqual(animalObj.speakLots(), "meow meow");
animalObj.species = "dog";
console.assertEqual(animalObj.speak(), "woof");
console.assertEqual(animalObj.speakLots(), "woof woof");
