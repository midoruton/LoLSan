
pub enum ObsidianURIAction{
    Open(OpenParam),
    Search(SearchParam),
    New(NewParam),
}

struct NewParam{
    vault: String,
    file: String,
    content: Option<String>,
    silent: Option<bool>,
}

pub struct NewBuilder {
    vault: String,
    file: String,
    content: Option<String>,
    silent: Option<bool>,
}

impl NewBuilder {
    pub fn vault(mut self, vault: String) -> Self {
        self.vault = vault;
        self
    }

    pub fn file(mut self, file: String) -> Self {
        self.file = file;
        self
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn silent(mut self, silent: bool) -> Self {
        self.silent = Some(silent);
        self
    }

    pub fn new(file:&str) -> Self {
        NewBuilder {
            vault: default_obsidian_vault(),
            file: file.to_string(),
            content: None,
            silent: None,
        }
    }

    pub fn build(self) -> ObsidianURIAction {
        ObsidianURIAction::New(NewParam {
            vault: self.vault,
            file: self.file,
            content: self.content,
            silent: self.silent,
        })
    }
}


struct OpenParam{
    vault: String,
    file: Option<String>,
}

pub struct OpenBuilder {
    param : OpenParam,
}

impl OpenBuilder {
    pub fn vault(mut self, vault: String) -> Self {
        self.param.vault = vault;
        self
    }

    pub fn file(mut self, file: String) -> Self {
        self.param.file = Some(file);
        self
    }

    pub fn new() -> Self {
        OpenBuilder {
            param: OpenParam {
                vault: default_obsidian_vault(),
                file: None,
            }
        }
    }

    pub fn build(self) -> ObsidianURIAction {
        ObsidianURIAction::Open(self.param)
    }
}

struct SearchParam{
    vault: String,
    query: Option<String>,
}

pub struct SearchBuilder {
    param : SearchParam,
}

impl SearchBuilder {
    pub fn vault(mut self, vault: String) -> Self {
        self.param.vault = vault;
        self
    }

    pub fn query(mut self, query: String) -> Self {
        self.param.query = Some(query);
        self
    }

    pub fn new() -> Self {
        SearchBuilder {
            param: SearchParam {
                vault: default_obsidian_vault(),
                query: None,
            }
        }
        
    }

    pub fn build(self) -> ObsidianURIAction {
        ObsidianURIAction::Search(self.param)
    }
}

impl ObsidianURIAction{
    fn encode(string:&String)->String{
        percent_encoding::utf8_percent_encode(string.as_str(), percent_encoding::NON_ALPHANUMERIC).to_string()
    }
    pub fn to_uri(&self) -> String {
        match self {
            ObsidianURIAction::Open(param) => {
                match &param.file {
                    Some(file) => {
                        format!("obsidian://open?vault={}&file={}", Self::encode(&param.vault), Self::encode(file))
                    }
                    None => {
                        format!("obsidian://open?vault={}", Self::encode(&param.vault))
                    }
                }
            }
            ObsidianURIAction::Search(param) => {
                match &param.query {
                    Some(query) => {
                        format!("obsidian://search?vault={}&query={}", Self::encode(&param.vault), Self::encode(query))
                    }
                    None => {
                        format!("obsidian://search?vault={}", Self::encode(&param.vault))
                    }
                }
            }
            ObsidianURIAction::New(param) => {
                let mut uri = format!("obsidian://new?vault={}&file={}", param.vault, param.file);
                if let Some(content) = &param.content {
                    uri.push_str(&format!("&content={}", content));
                }
                if let Some(silent) = param.silent {
                    uri.push_str(&format!("&silent={}", silent));
                }
                uri
            }
        }
    }
}


use std::path::{PathBuf};
pub fn obsidian_file_exists(file_path: &PathBuf) -> bool {
    file_path.exists()
}
mod tests {
    use super::*;
    #[test]
    fn test_open_builder() {
        let action = OpenBuilder::new().build();
        assert_eq!(action.to_uri(), "obsidian://open?vault=LeagueOfLegends");
        let action = OpenBuilder::new().vault("my vault".to_string()).build();
        assert_eq!(action.to_uri(), "obsidian://open?vault=my%20vault");
        let action = OpenBuilder::new().vault("ef6ca3e3b524d22f".to_string()).build();
        assert_eq!(action.to_uri(), "obsidian://open?vault=ef6ca3e3b524d22f");
        let action = OpenBuilder::new().vault("my vault".to_string()).file("my note".to_string()).build();
        assert_eq!(action.to_uri(), "obsidian://open?vault=my%20vault&file=my%20note");
    }

    #[test]
    fn test_search_builder() {
        let action = SearchBuilder::new().build();
        assert_eq!(action.to_uri(), "obsidian://search?vault=LeagueOfLegends");
        let action = SearchBuilder::new().vault("my vault".to_string()).query("MOC".to_string()).build();
        assert_eq!(action.to_uri(), "obsidian://search?vault=my%20vault&query=MOC");
    }

    #[test]
    fn test_new_builder() {
        let builder = NewBuilder::new("test.md");
        let action = builder.build();
        assert_eq!(action.to_uri(), "obsidian://new?vault=LeagueOfLegends&file=test.md");
    }

    #[test]
    fn test_new_builder_with_content() {
        let builder = NewBuilder::new("test.md").content("content".to_string());
        let action = builder.build();
        assert_eq!(action.to_uri(), "obsidian://new?vault=LeagueOfLegends&file=test.md&content=content");
    }

    #[test]
    fn test_new_builder_with_silent() {
        let builder = NewBuilder::new("test.md").silent(true);
        let action = builder.build();
        assert_eq!(action.to_uri(), "obsidian://new?vault=LeagueOfLegends&file=test.md&silent=true");
    }
}


fn default_obsidian_vault() -> String{
    "LeagueOfLegends".to_string()
}