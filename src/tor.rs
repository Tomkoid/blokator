use regex::Regex;

pub fn if_onion_link(url: String) -> bool {
    let regex = Regex::new(".*..onion").unwrap();
    regex.is_match(&url)
}
