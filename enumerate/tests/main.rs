#![feature(assert_matches)]

use enumerate::{Enumerate, EnumerateStr, NoSuchVariantError};
use std::assert_matches::assert_matches;
use std::str::FromStr;

#[test]
fn enumerate() {
    #[derive(Enumerate, Debug)]
    enum Empty {}

    #[derive(Enumerate, Debug)]
    enum Single {
        Item1,
    }

    #[derive(Enumerate, Debug)]
    enum Couple {
        Item1,
        Item2,
    }

    #[derive(Enumerate, Debug)]
    enum Triple {
        Item1,
        Item2,
        Item3,
    }

    assert_matches!(Empty::enumerate(), []);
    assert_matches!(Single::enumerate(), [Single::Item1]);
    assert_matches!(Couple::enumerate(), [Couple::Item1, Couple::Item2]);
    assert_matches!(
        Triple::enumerate(),
        [Triple::Item1, Triple::Item2, Triple::Item3]
    );
}

#[test]
fn enumerate_str() {
    #[derive(EnumerateStr, Clone, Debug)]
    enum Empty {}

    #[derive(EnumerateStr, Clone, Debug)]
    enum Single {
        Item1,
    }

    #[derive(EnumerateStr, Clone, Debug)]
    enum Couple {
        Item1,
        Item2,
    }

    #[derive(EnumerateStr, Clone, Debug)]
    enum Rename {
        #[enumerate_str(rename = "The first item...")]
        Item1,
        #[enumerate_str(rename = "The second item...")]
        Item2,
    }

    #[derive(EnumerateStr, Clone, Debug)]
    #[enumerate_str(rename_all = "lowercase")]
    enum RenameAll {
        Item1,
        Item2,
    }

    #[derive(EnumerateStr, Clone, Debug)]
    #[enumerate_str(rename_all = "UPPERCASE")]
    enum RenameMix {
        Item1,
        #[enumerate_str(rename = "The second item...")]
        Item2,
    }

    assert_matches!(Empty::from_str("Gibberish"), Err(NoSuchVariantError));
    assert_matches!(Empty::from_str(""), Err(NoSuchVariantError));

    assert_eq!(Single::Item1.as_str(), "Item1");
    assert_matches!(Single::from_str("Item1"), Ok(Single::Item1));
    assert_matches!(Single::from_str("Gibberish"), Err(NoSuchVariantError));
    assert_matches!(Single::from_str(""), Err(NoSuchVariantError));

    assert_eq!(Couple::Item1.as_str(), "Item1");
    assert_eq!(Couple::Item2.as_str(), "Item2");
    assert_matches!(Couple::from_str("Item1"), Ok(Couple::Item1));
    assert_matches!(Couple::from_str("Item2"), Ok(Couple::Item2));
    assert_matches!(Couple::from_str("item1"), Err(NoSuchVariantError));
    assert_matches!(Couple::from_str("item2"), Err(NoSuchVariantError));
    assert_matches!(Couple::from_str("ITEM1"), Err(NoSuchVariantError));
    assert_matches!(Couple::from_str("ITEM2"), Err(NoSuchVariantError));
    assert_matches!(Couple::from_str("Gibberish"), Err(NoSuchVariantError));
    assert_matches!(Couple::from_str(""), Err(NoSuchVariantError));

    assert_eq!(Rename::Item1.as_str(), "The first item...");
    assert_eq!(Rename::Item2.as_str(), "The second item...");
    assert_matches!(Rename::from_str("The first item..."), Ok(Rename::Item1));
    assert_matches!(Rename::from_str("The second item..."), Ok(Rename::Item2));
    assert_matches!(
        Rename::from_str("the first item..."),
        Err(NoSuchVariantError)
    );
    assert_matches!(
        Rename::from_str("the second item..."),
        Err(NoSuchVariantError)
    );
    assert_matches!(
        Rename::from_str("THE FIRST ITEM..."),
        Err(NoSuchVariantError)
    );
    assert_matches!(
        Rename::from_str("THE SECOND ITEM..."),
        Err(NoSuchVariantError)
    );
    assert_matches!(Rename::from_str("Gibberish"), Err(NoSuchVariantError));
    assert_matches!(Rename::from_str(""), Err(NoSuchVariantError));

    assert_eq!(RenameAll::Item1.as_str(), "item1");
    assert_eq!(RenameAll::Item2.as_str(), "item2");
    assert_matches!(RenameAll::from_str("item1"), Ok(RenameAll::Item1));
    assert_matches!(RenameAll::from_str("item2"), Ok(RenameAll::Item2));
    assert_matches!(RenameAll::from_str("Item1"), Err(NoSuchVariantError));
    assert_matches!(RenameAll::from_str("Item2"), Err(NoSuchVariantError));
    assert_matches!(RenameAll::from_str("ITEM1"), Err(NoSuchVariantError));
    assert_matches!(RenameAll::from_str("ITEM2"), Err(NoSuchVariantError));
    assert_matches!(RenameAll::from_str("Gibberish"), Err(NoSuchVariantError));
    assert_matches!(RenameAll::from_str(""), Err(NoSuchVariantError));

    assert_eq!(RenameMix::Item1.as_str(), "ITEM1");
    assert_eq!(RenameMix::Item2.as_str(), "The second item...");
    assert_matches!(RenameMix::from_str("ITEM1"), Ok(RenameMix::Item1));
    assert_matches!(
        RenameMix::from_str("The second item..."),
        Ok(RenameMix::Item2)
    );
    assert_matches!(RenameMix::from_str("item1"), Err(NoSuchVariantError));
    assert_matches!(
        RenameMix::from_str("the second item..."),
        Err(NoSuchVariantError)
    );
    assert_matches!(
        RenameMix::from_str("THE SECOND ITEM..."),
        Err(NoSuchVariantError)
    );
    assert_matches!(RenameMix::from_str("Gibberish"), Err(NoSuchVariantError));
    assert_matches!(RenameMix::from_str(""), Err(NoSuchVariantError));
}
