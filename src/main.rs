use std::{collections::{BTreeMap, HashMap}, time::Instant};

// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;
use uuid::Uuid;
use views::{Chat, Home, Navbar};

/// Define a components module that contains all shared components for our app.
mod components;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
/// 
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},

        #[route("/blog/:id")]
        Chat { id: Uuid },
}

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

#[derive(Clone)]
pub struct Message {
    pub id: Instant,
    pub author: Uuid,
    pub content: String,
}

#[derive(Clone)]
pub struct Author {
    pub id: Uuid,
    pub username: String,
    pub avatar: Option<String>,
    pub bio: Option<String>,
}

pub type Messages = BTreeMap::<Instant, Message>;
pub type Authors = HashMap::<Uuid, Author>;

#[derive(Clone)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub messages: Messages,
}

pub type Rooms = BTreeMap::<Uuid, Room>;

pub static AUTHORS: GlobalSignal<Authors> = Global::new(|| Authors::new());
pub static ROOMS: GlobalSignal<Rooms> = Global::new(|| Rooms::new());

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    let anakin = Author {
        id: Uuid::new_v4(),
        username: "Anakin".to_string(),
        avatar: Some("https://img.daisyui.com/images/profile/demo/anakeen@192.webp".to_string()),
        bio: Some("Badguy".to_string()),
    };

    let obi = Author {
        id: Uuid::new_v4(),
        username: "Obi-Wan Kenobi".into(),
        avatar: Some("https://img.daisyui.com/images/profile/demo/kenobee@192.webp".into()),
        bio: Some("Jedi".to_string()),
    };

    let yoda = Author {
        id: Uuid::new_v4(),
        username: "Yoda".into(),
        avatar: Some("https://img.daisyui.com/images/profile/demo/averagebulk@192.webp".into()),
        bio: Some("Jedi".to_string()),
    };

    let luke = Author {
        id: Uuid::new_v4(),
        username: "Luke Skywalker".into(),
        avatar: Some("https://img.daisyui.com/images/profile/demo/superperson@192.webp".into()),
        bio: Some("Superhero".to_string()),
    };

    let scarlett = Author {
        id: Uuid::new_v4(),
        username: "Scarlett Johansson".into(),
        avatar: Some("https://img.daisyui.com/images/profile/demo/distracted2@192.webp".into()),
        bio: Some("Human".to_string()),
    };

    let mark = Author {
        id: Uuid::new_v4(),
        username: "Mark Ruffalo".into(),
        avatar: Some("https://img.daisyui.com/images/profile/demo/distracted1@192.webp".into()),
        bio: Some("Superhero".to_string()),
    };

    let mut messages = Messages::new();

    for _ in 0..10 {
        let ts = Instant::now();

        messages.insert(ts, Message {
            id: ts,
            author: obi.id,
            content: "You were the Chosen One!".into(),
        });
        
        let ts = Instant::now();

        messages.insert(ts, Message {
            id: ts,
            author: anakin.id,
            content: "I hate you!".into(),
        });

        let ts = Instant::now();

        messages.insert(ts, Message {
            id: ts,
            author: yoda.id,
            content: "Do or do not. There is no try.".into(),
        });

        let ts = Instant::now();

        messages.insert(ts, Message {
            id: ts,
            author: luke.id,
            content: "I'll never turn to the dark side.".into(),
        });
    }
    
    {
        let mut authors = AUTHORS.write();

        authors.insert(obi.id, obi);
        authors.insert(yoda.id, yoda);
        authors.insert(luke.id, luke);
        authors.insert(scarlett.id, scarlett);
        authors.insert(mark.id, mark);
    }

    let starwars = Room {
        id: Uuid::new_v4(),
        name: "Starwars".to_string(),
        messages,
    };

    let spiderman = Room {
        id: Uuid::new_v4(),
        name: "Spiderman".to_string(),
        messages: Messages::new(),
    };

    let batman = Room {
        id: Uuid::new_v4(),
        name: "Batman".to_string(),
        messages: Messages::new(),
    };

    let hulk = Room {
        id: Uuid::new_v4(),
        name: "Hulk".to_string(),
        messages: Messages::new(),
    };

    let avengers = Room {
        id: Uuid::new_v4(),
        name: "Avengers".to_string(),
        messages: Messages::new(),
    };

    {
        let mut rooms = ROOMS.write();

        rooms.insert(starwars.id, starwars);
        rooms.insert(spiderman.id, spiderman);
        rooms.insert(batman.id, batman);
        rooms.insert(hulk.id, hulk);
        rooms.insert(avengers.id, avengers);
    }

    use_context_provider(|| anakin);

    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // The router component renders the route enum we defined above. It will handle synchronization of the URL and render
        // the layouts and components for the active route.
        Router::<Route> {}
    }
}
