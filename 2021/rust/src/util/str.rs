pub fn str_is_uppercase(s: &str) -> bool {
    s.chars().all(|c| c.is_uppercase())
}
