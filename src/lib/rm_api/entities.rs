#[derive(Default, Clone)]
pub struct Page<T> {
    pub info: Info,
    pub results: Vec<T>,
}

#[derive(Default, Clone)]
pub struct Info {
    pub count: u32,
    pub pages: u32,
    pub next: Option<String>,
    pub prev: Option<String>,
}

#[derive(Default,Clone)]
pub struct Location {
    pub id: u32,
    pub name: String,
    pub _type: String,
    pub dimension: String,
    pub residents: Vec<String>,
    pub url: String,
    pub created: String,
}

#[derive(Default,Clone)]
pub struct Episode {
    pub id: u32,
    pub name: String,
    pub air_date: String,
    pub episode: String,
    pub characters: Vec<String>,
    pub url: String,
    pub created: String,
}

#[derive(Default,Clone)]
pub struct Character {
    pub id: u32,
    pub name: String,
    pub status: String,
    pub species: String,
    pub _type: String,
    pub gender: String,
    pub image: String,
    pub episode: Vec<String>,
    pub url: String,
    pub created: String,
}

pub type CharacterPage = Page<Character>;
pub type EpisodePage = Page<Episode>;
pub type LocationPage = Page<Location>;
