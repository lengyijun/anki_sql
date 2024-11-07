use std::env;

use anki::decks::DeckId;
use anki::notetype::NoteField;
use anki::prelude::{Notetype, NotetypeId};
use anki::{collection::CollectionBuilder, notes::Note};
use anyhow::Result;

const FIELD_COUNT: usize = 2;

fn main() -> Result<()> {
    let word = env::args().nth(1).unwrap();

    let p = dirs::home_dir()
        .unwrap()
        .join(".local/share/Anki2")
        .join("User 1")
        .join("collection.anki2");
    let mut builder = CollectionBuilder::new(p);
    let mut collection = builder.build()?;
    let mut note: Note = Note::new(&Notetype {
        id: NotetypeId::from(1703237851910),
        fields: vec![NoteField::new("hello"); FIELD_COUNT],
        name: Default::default(),
        mtime_secs: Default::default(),
        usn: Default::default(),
        templates: Default::default(),
        config: Default::default(),
    });
    note.set_field(0, word)?;

    collection.add_note(&mut note, DeckId::from(1730334088979))?;
    Ok(())
}
