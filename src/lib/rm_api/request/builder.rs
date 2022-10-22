

pub trait RequestURLBuilder {
    fn build_url(&self) -> String;
}

pub struct CharactersRequest(Vec<String>);

impl CharactersRequest {
    pub fn new(domain: &str) -> Self {
        CharactersRequest(vec![format!(
            "{}{}",
            domain.to_string(),
            "/api/character/?".to_string()
        )])
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.0.push(format!("name={}", name));
        self
    }
}

impl RequestURLBuilder for CharactersRequest {
    fn build_url(&self) -> String {
        let first = self.0.first().unwrap();
        let rest = self.0[1..].join("&");
        format!("{}{}", first, rest)
    }
}

#[derive(Clone)]
pub struct EpisodesRequest(Vec<String>);
impl EpisodesRequest {
    pub fn new(domain: &str, episode_numbers: Vec<u32>) -> Self {
        let main_url = format!("{}{}", domain, "/api/episode/");
        let episode_part = episode_numbers
            .into_iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(",");
        EpisodesRequest(vec![format!("{}{}{}", main_url, episode_part, "?")])
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.0.push(format!("name={}", name));
        self
    }
}

impl RequestURLBuilder for EpisodesRequest {
    fn build_url(&self) -> String {
        let first = self.0.first().unwrap();
        let rest = self.0[1..].join("&");
        format!("{}{}", first, rest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_characters_request() {
        let url = CharactersRequest::new("https://rickandmortyapi.com")
            .name("rick")
            .build_url();
        assert_eq!(url, "https://rickandmortyapi.com/api/character/?name=rick");
    }

    #[test]
    fn test_episodes_request() {
        let url = EpisodesRequest::new("https://rickandmortyapi.com", vec![1, 2, 3])
            .name("rick")
            .build_url();
        assert_eq!(
            url,
            "https://rickandmortyapi.com/api/episode/1,2,3?name=rick"
        );
    }

    #[test]
    fn calling_name_twice_should_overwrite() {
        let url = EpisodesRequest::new("https://rickandmortyapi.com", vec![1, 2, 3])
            .name("rick")
            .name("morty")
            .build_url();
        assert_eq!(
            url,
            "https://rickandmortyapi.com/api/episode/1,2,3?name=morty"
        );
    }
}
