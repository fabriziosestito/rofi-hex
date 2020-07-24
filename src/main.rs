use rustofi::components::EntryBox;
use rustofi::components::ItemList;
use rustofi::RustofiResult;
use serde::Deserialize;
use serde_json::error::Error as SerdeError;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

fn search_entry() -> RustofiResult {
    EntryBox::display("Hexdocs: ".to_string())
}

#[derive(Deserialize, Debug, Clone)]
struct HexPackage {
    docs_html_url: Option<String>,
    name: String,
}

impl std::fmt::Display for HexPackage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

enum RofiHexError {
    RequestError(reqwest::Error),
    DeserializationError(SerdeError),
}

impl From<reqwest::Error> for RofiHexError {
    fn from(error: reqwest::Error) -> Self {
        RofiHexError::RequestError(error)
    }
}

impl From<SerdeError> for RofiHexError {
    fn from(error: SerdeError) -> Self {
        RofiHexError::DeserializationError(error)
    }
}

fn query_hex_pm(query: String) -> Result<RustofiResult, RofiHexError> {
    let query_url = format!(
        "https://hex.pm/api/packages?search={query}&sort=recent_downloads",
        query = query
    );

    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap_or_else(|_| panic!("Failed to build reqwest client"));
    let res = client.get(&query_url).send()?;
    let hex_packages: Vec<HexPackage> = res.json()?;

    Ok(ItemList::new(hex_packages, Box::new(simple_callback))
        .display("Select an entry".to_string()))
}

fn simple_callback(h: &HexPackage) -> RustofiResult {
    if let Some(url) = &h.docs_html_url {
        if webbrowser::open(url).is_ok() {
            return RustofiResult::Exit;
        }
    }

    RustofiResult::Error
}

fn main() {
    loop {
        match search_entry() {
            RustofiResult::Error => break,
            RustofiResult::Exit => break,
            RustofiResult::Cancel => break,
            RustofiResult::Blank => break,
            RustofiResult::Selection(selection) => {
                if let Ok(RustofiResult::Exit) = query_hex_pm(selection) {
                    break;
                }
            }
            _ => {}
        }
    }
}
