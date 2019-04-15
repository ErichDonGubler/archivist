mod archivist {
    pub mod deps {
        pub use {chrono, url, vec1};
    }

    pub mod model {
        use {
            chrono::{DateTime, FixedOffset},
            derive_more::Display,
            std::{collections::BTreeMap, ops::Deref, path::PathBuf},
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

        impl Deref for Prose {
            type Target = str;

            fn deref(&self) -> &str {
                &*self.0
            }
        }

        #[derive(Clone, Debug)]
        pub struct LinkElement {
            pub content: Option<Box<SpanElement>>,
            pub kind: LinkElementKind,
        }

        #[derive(Clone, Debug)]
        pub enum LinkElementKind {
            Url(Url),
            // Person(PersonId),
            // Place(PlaceId),
            // Tag(TagId),
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
            Link(LinkElement),
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
            Unordered,
            Ordered,
        }

        #[derive(Clone, Debug)]
        pub struct ListElement {
            pub marker: ListElementMarker,
            pub items: Vec1<Vec1<BlockElement>>,
        }

        #[derive(Clone, Copy, Debug)]
        pub enum HeaderLevel {
            One,
            Two,
            Three,
            Four,
            Five,
            Six,
        }

        #[derive(Clone, Debug)]
        pub struct HeaderElement {
            pub text: SpanElement,
            pub level: HeaderLevel,
        }

        #[derive(Clone, Debug)]
        pub enum BlockElement {
            Paragraph(SpanElement),
            Media(MediaElement),
            BlockQuote(SpanElement),
            List(ListElement),
            Header(HeaderElement),
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
                    // FIXME: This is NOT correct. Got this from:
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
            // pub notes: Vec<Note>,
            // pub tags: Vec<TagId>,
            // pub places: Vec<PlaceId>, // TODO: Should this just be a single place?
            // pub people: Vec<PersonId>,
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
            // pub tags: Vec<TagId>,
            // pub places: Vec<PlaceId>,
            // pub people: Vec<PersonId>,
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
            pub entries: BTreeMap<EntryId, Entry>,
            // pub notes: BTreeMap<NoteId, NoteEntry>,
            // pub tags: BTreeMap<TagId, Tag>,
            // pub places: BTreeMap<PlaceId, Place>,
            // pub people: BTreeMap<PersonId, Person>,
        }
    }
}

mod html {
    use {
        crate::archivist::model::{
            BlockElement, ContentDates, Entry, EntryId, HeaderElement, HeaderLevel, Identifier,
            Journal, LinkElement, LinkElementKind, ListElement, ListElementMarker, MediaElement,
            MediaReference, Prose, SpanElement,
        },
        askama_escape::{Html, MarkupDisplay},
        std::fmt::{Display, Formatter, Result as FmtResult},
        vec1::Vec1,
    };

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

    impl DisplayAsHtml for Journal {
        fn fmt(&self, f: &mut Formatter) -> FmtResult {
            let Self {
                entries,
                // notes,
                // tags,
                // places,
                // people,
            } = self;

            for (
                entry_id,
                Entry {
                    title,
                    subtitle,
                    content_dates:
                        ContentDates {
                            created,
                            last_modified,
                        },
                    elements,
                    // notes,
                    // tags,
                    // places,
                    // people,
                },
            ) in entries.iter()
            {
                write!(f, "<header data-id=\"{}\">", &*entry_id)?;
                {
                    write!(f, "<h2>{}</h2>", title)?;
                    if let Some(subtitle) = subtitle {
                        write!(f, "<div class=\"subtitle\">{}</h2>", subtitle)?;
                    }
                    write!(f, "<div class=\"date\">{}</div>", created)?;
                    write!(
                        f,
                        "<div class=\"date\">Last modified: {}</div>",
                        last_modified
                    )?;
                }
                write!(f, "</header>")?;
                write!(f, "<article>")?;
                for element in elements.iter() {
                    write!(f, "{}", element.html())?;
                }
                write!(f, "</article>")?;
            }

            Ok(())
        }
    }

    impl DisplayAsHtml for BlockElement {
        fn fmt(&self, f: &mut Formatter) -> FmtResult {
            use BlockElement::*;

            match self {
                Paragraph(span) => write!(f, "<p>{}</p>", span.html()),
                Media(media) => write!(f, "{}", media.html()),
                BlockQuote(span) => write!(f, "<blockquote>{}</blockquote>", span.html()),
                List(list) => write!(f, "{}", list.html()),
                Header(header) => write!(f, "{}", header.html()),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct AsSrcAttr<'a>(pub &'a MediaReference);

    impl Display for AsSrcAttr<'_> {
        fn fmt(&self, f: &mut Formatter) -> FmtResult {
            use MediaReference::*;

            match self.0 {
                Link(url) => write!(f, "{}", MarkupDisplay::new_unsafe(url.as_str(), Html)),
                LocalPath(path) => write!(f, "{}", MarkupDisplay::new_unsafe(path.display(), Html)),
            }
        }
    }

    impl DisplayAsHtml for SpanElement {
        fn fmt(&self, f: &mut Formatter) -> FmtResult {
            use SpanElement::*;

            match self {
                PhysicalSourcePageBegin {
                    page_index,
                    source_photo,
                } => {
                    write!(
                        f,
                        "<span class=\"source-page-ref\" data-idx=\"{}\"",
                        page_index
                    )?;
                    if let Some(source_photo) = source_photo {
                        write!(f, " data-src=\"{}\"", AsSrcAttr(source_photo))?;
                    }
                    write!(f, "></span>")?;
                    Ok(())
                }
                Text(t) => write!(f, "{}", MarkupDisplay::new_unsafe(&*t, Html)),
                Emphasis(span) => write!(f, "<em>{}</em>", span.html()),
                Strong(span) => write!(f, "<strong>{}</strong>", span.html()),
                Strikethrough(span) => write!(f, "<del>{}</del>", span.html()), // XXX: Is this correct? HTML5 can treat this specially,
                Link(link) => {
                    use LinkElementKind::*;

                    let LinkElement { kind, content } = link;
                    match kind {
                        Url(url) => {
                            let link_writer = || MarkupDisplay::new_unsafe(url.as_str(), Html);

                            write!(f, "<a href=\"{}\">", link_writer())?;
                            if let Some(content) = content {
                                write!(f, "{}", content.html())?;
                            } else {
                                write!(f, "{}", link_writer())?;
                            }
                            write!(f, "</a>")
                        }
                    }
                }
                Group(spans) => {
                    for span in spans.iter() {
                        write!(f, "{}", span.html())?;
                    }
                    Ok(())
                }
            }
        }
    }

    impl DisplayAsHtml for ListElement {
        fn fmt(&self, f: &mut Formatter) -> FmtResult {
            use ListElementMarker::*;

            let Self { marker, items } = self;

            let list_type_char = match marker {
                Unordered => 'u',
                Ordered => 'o',
            };

            write!(f, "<{}l>", list_type_char)?;
            for block_item_list in items.iter() {
                write!(f, "<li>")?;
                for block_item in block_item_list.iter() {
                    write!(f, "{}", block_item.html())?;
                }
                write!(f, "</li>")?;
            }
            write!(f, "</{}l>", list_type_char)
        }
    }

    impl DisplayAsHtml for MediaElement {
        fn fmt(&self, f: &mut Formatter) -> FmtResult {
            use MediaElement::*;

            match self {
                Image(media_ref) => write!(f, "<img src=\"{}\">", AsSrcAttr(media_ref)),
                Video(media_ref) => write!(
                    f,
                    "<audio controls src=\"{}\"></audio>",
                    AsSrcAttr(media_ref)
                ),
                Audio(media_ref) => write!(
                    f,
                    "<video controls src=\"{}\"></video>",
                    AsSrcAttr(media_ref)
                ),
            }
        }
    }

    impl DisplayAsHtml for HeaderElement {
        fn fmt(&self, f: &mut Formatter) -> FmtResult {
            use HeaderLevel::*;

            let Self { text, level } = self;

            let level = match level {
                One => 1,
                Two => 2,
                Three => 3,
                Four => 4,
                Five => 5,
                Six => 6,
            };

            write!(f, "<h{level}>{}</h{level}>", text.html(), level = level)
        }
    }
}

use {
    self::html::DisplayAsHtml,
    archivist::model::{
        BlockElement, ContentDates, Entry, EntryId, Identifier, Journal, Prose, SpanElement,
    },
    vec1::Vec1,
};

fn main() {
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
            // notes: Vec::new(),
            // tags: Vec::new(),
            // places: Vec::new(),
            // people: Vec::new(),
        },
    );

    let asdf = r#"
+++
id = "wat"
title = "hay sup"
subtitle = "nuttin much"
content_dates = {
    last_modified: 2019-04-13 20:22:01 +00:00
    created: 2019-04-13 20:22:01 +00:00
}
+++

asdf

"#;
    println!("{}", journal.html());
}


