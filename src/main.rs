mod archivist {
    pub mod deps {
        pub use {chrono, url, vec1};
    }

    pub mod model {
        use {
            chrono::{DateTime, FixedOffset},
            derive_more::Display,
            std::{collections::BTreeMap, path::PathBuf},
            url::Url,
            vec1::Vec1,
        };

        /// A [`String`] with at least a single grapheme cluster.
        #[derive(Clone, Debug, Display, Eq, Hash, PartialEq, PartialOrd, Ord)]
        pub struct Prose(String);

        impl Prose {
            #[deprecated(
                since = "0.0.0",
                note = "This needs to be replaced once constraints are established."
            )]
            pub fn new(s: String) -> Self {
                Self(s)
            }
        }

        #[derive(Clone, Debug)]
        pub enum SpanElement {
            PhysicalSourcePageBegin {
                page_index: u32,
                source_photo: Option<MediaReference>,
            },
            Text(Prose),
            Emphasis(Box<SpanElement>),
            Strong(Box<SpanElement>),
            Strikethrough(Box<SpanElement>),
            UrlLink(Url),
            PersonLink(PersonId),
            PlaceLink(PlaceId),
            TagLink(TagId),
            Group(Vec1<SpanElement>),
        }

        #[derive(Clone, Debug)]
        pub enum MediaReference {
            Link(Url),
            LocalPath(PathBuf),
        }

        #[derive(Clone, Debug)]
        pub enum MediaElement {
            Image(MediaReference),
            Video(MediaReference),
            Audio(MediaReference),
        }

        #[derive(Clone, Debug)]
        pub enum ListElementMarker {
            Number { start: u32 },
            Letter { start: u32, uppercase: bool },
            RomanNumeral { start: u32, uppercase: bool },
            Custom(char),
        }

        #[derive(Clone, Debug)]
        pub struct ListElement {
            sigil: Option<ListElementMarker>,
            items: Vec1<Vec1<BlockElement>>,
        }

        #[derive(Clone, Debug)]
        pub enum BlockElement {
            Paragraph(SpanElement),
            Media(MediaElement),
            BlockQuote(SpanElement),
            List(ListElement),
            Header,
        }

        #[derive(Clone, Debug)]
        pub struct ContentDates {
            pub created: DateTime<FixedOffset>,
            pub last_modified: DateTime<FixedOffset>,
        }

        impl ContentDates {
            pub fn now() -> Self {
                use chrono::Local;
                Self {
                    // FIXME: This LOOKS correct? Got this from:
                    // https://github.com/chronotope/chrono/pull/271/files#diff-8daac88609c52af7718d9f334e7a2bf7R293
                    created: Local::now().with_timezone(&FixedOffset::east(0)),
                    last_modified: Local::now().with_timezone(&FixedOffset::east(0)),
                }
            }
        }

        /// TODO: Constraints?
        ///
        /// The only thing that might make sense here is Unicode normalization and whitespace trimming.
        #[derive(Clone, Debug, Display, Eq, Hash, PartialEq, PartialOrd, Ord)]
        pub struct Identifier(Prose);

        impl Identifier {
            #[deprecated(
                since = "0.0.0",
                note = "This needs to be replaced once constraints are established."
            )]
            pub fn new(s: String) -> Self {
                Self(Prose::new(s))
            }
        }

        #[derive(Clone, Debug, Display, Eq, Hash, PartialEq, PartialOrd, Ord)]
        pub struct EntryId(pub Identifier);

        #[derive(Clone, Debug, Display, Eq, Hash, PartialEq, PartialOrd, Ord)]
        pub struct TagId(pub Identifier);

        #[derive(Clone, Debug, Display, Eq, Hash, PartialEq, PartialOrd, Ord)]
        pub struct PlaceId(pub Identifier);

        #[derive(Clone, Debug, Display, Eq, Hash, PartialEq, PartialOrd, Ord)]
        pub struct PersonId(pub Identifier);

        #[derive(Clone, Debug)]
        pub struct Tag {
            name: Option<Prose>,
            description: Option<Prose>,
        }

        #[derive(Clone, Debug)]
        pub struct Place {
            name: Option<Prose>,
            description: Option<Prose>,
            location: PhysicalLocation,
        }

        #[derive(Clone, Debug)]
        pub struct Person {
            name: Prose,
            description: Option<Prose>,
        }

        #[derive(Clone, Debug)]
        pub struct Entry {
            pub title: Prose,
            pub subtitle: Option<Prose>,
            pub content_dates: ContentDates,
            pub elements: Vec1<BlockElement>,
            pub notes: Vec<Note>,
            pub tags: Vec<TagId>,
            pub places: Vec<PlaceId>, // TODO: Should this just be a single place?
            pub people: Vec<PersonId>,
        }

        /// Subset of `String` of non-line characters separated by newlines.
        #[derive(Clone, Debug)]
        pub struct Address(String);

        #[derive(Clone, Debug)]
        pub struct DecimicroGeocoordinate(i32);

        #[derive(Clone, Debug)]
        pub struct GeoCoordinates {
            latitude: DecimicroGeocoordinate,
            longitude: DecimicroGeocoordinate,
        }

        #[derive(Clone, Debug)]
        pub enum PhysicalLocation {
            Address(Address),
            Coordinates(GeoCoordinates),
        }

        #[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
        pub struct NoteId(Identifier);

        #[derive(Clone, Debug)]
        pub struct NoteEntry {
            note: Note,
            target_entry: EntryId,
        }

        #[derive(Clone, Debug)]
        pub struct Note {
            pub span: EntryContentSpan,
            pub content_dates: ContentDates,
            pub text: Vec1<BlockElement>,
            pub tags: Vec<TagId>,
            pub places: Vec<PlaceId>,
            pub people: Vec<PersonId>,
        }

        #[derive(Clone, Debug)]
        pub struct EntryContentLocation {
            paragraph_idx: u32,
            word_idx: u32,
        }

        /// Inclusive, so start and end are both used here
        #[derive(Clone, Debug)]
        pub struct EntryContentSpan {
            pub start: EntryContentLocation,
            pub end: EntryContentLocation,
        }

        #[derive(Clone, Debug)]
        pub struct EntryContentReference {
            pub id: EntryId,
            pub span: EntryContentSpan,
        }

        #[derive(Clone, Debug, Default)]
        pub struct Journal {
            pub notes: BTreeMap<NoteId, NoteEntry>,
            pub entries: BTreeMap<EntryId, Entry>,
            pub tags: BTreeMap<TagId, Tag>,
            pub places: BTreeMap<PlaceId, Place>,
            pub people: BTreeMap<PersonId, Person>,
        }
    }

    pub use self::model::Journal;
}

mod html {
    use std::fmt::{Display, Formatter, Result as FmtResult};

    pub trait DisplayAsHtml {
        fn fmt(&self, f: &mut Formatter) -> FmtResult;

        fn html(&self) -> HtmlDisplay<&Self> {
            HtmlDisplay(self)
        }
    }

    impl<T> DisplayAsHtml for &T
    where
        T: DisplayAsHtml,
    {
        fn fmt(&self, f: &mut Formatter) -> FmtResult {
            <T as DisplayAsHtml>::fmt(self, f)
        }
    }

    impl<T> DisplayAsHtml for &mut T
    where
        T: DisplayAsHtml,
    {
        fn fmt(&self, f: &mut Formatter) -> FmtResult {
            <T as DisplayAsHtml>::fmt(self, f)
        }
    }

    #[derive(Clone, Debug)]
    pub struct HtmlDisplay<T>(T);

    impl<T> Display for HtmlDisplay<T>
    where
        T: DisplayAsHtml,
    {
        fn fmt(&self, f: &mut Formatter) -> FmtResult {
            <T as DisplayAsHtml>::fmt(&self.0, f)
        }
    }
}

use {
    archivist::{Entry, Journal},
    html::DisplayAsHtml,
    std::fmt::{Formatter, Result as FmtResult},
};

impl DisplayAsHtml for Journal {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let Self {
            notes,
            entries,
            tags,
            places,
            people,
        } = self;

        write!(f, "Journal time!\n")?;

        for (
            entry_id,
            Entry {
                title,
                subtitle,
                content_dates,
                elements,
                notes,
                tags,
                places,
                people,
            },
        ) in entries.iter()
        {
            write!(f, "Entry ID {}:\n\n", entry_id)?;
        }

        Ok(())
    }
}

fn main() {
    use {
        archivist::model::{
            BlockElement, ContentDates, Entry, EntryId, Identifier, Prose, SpanElement,
        },
        vec1::Vec1,
    };

    let mut journal = Journal::default();

    journal.entries.insert(
        EntryId(Identifier::new("wat".to_owned())),
        Entry {
            title: Prose::new("hay sup".to_owned()),
            subtitle: Some(Prose::new("nuttin much".to_owned())),
            content_dates: ContentDates::now(),
            elements: Vec1::new(BlockElement::Paragraph(SpanElement::Text(Prose::new(
                "asdf".to_owned(),
            )))),
            notes: Vec::new(),
            tags: Vec::new(),
            places: Vec::new(),
            people: Vec::new(),
        },
    );
    println!("{}", Journal::default().html());
}
