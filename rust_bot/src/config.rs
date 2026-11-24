//! Faithful Rust ports of KYOKA's environment/config parsing helpers.
use std::env;

pub fn token() -> Result<String, &'static str> {
    ["DISCORD_TOKEN", "BOT_TOKEN", "TOKEN"].iter()
        .find_map(|key| env::var(key).ok().filter(|v| !v.trim().is_empty()))
        .map(|v| v.trim().to_owned())
        .ok_or("Discord token missing (DISCORD_TOKEN, BOT_TOKEN, or TOKEN)")
}
pub fn prefix() -> String {
    env::var("BOT_PREFIX").or_else(|_| env::var("DEFAULT_PREFIX"))
        .unwrap_or_else(|_| "+".to_string())
}
pub fn parse_application_id(raw: &str) -> Option<u64> {
    if raw.is_empty() || raw.len() > 20 || !raw.bytes().all(|b| b.is_ascii_digit()) { return None; }
    let n = raw.parse::<u64>().ok()?;
    (n > 0).then_some(n)
}
pub fn parse_shard_ids(raw: &str) -> Result<Vec<u64>, String> {
    if raw.trim().is_empty() { return Ok(Vec::new()); }
    let mut ids = Vec::new();
    for token in raw.split(',') {
        let t = token.trim();
        if t.is_empty() || !t.bytes().all(|b| b.is_ascii_digit()) {
            return Err(format!("Invalid shard ID {t:?}; use comma-separated non-negative integers."));
        }
        let id = t.parse::<u64>().map_err(|_| format!("Invalid shard ID {t:?}"))?;
        if ids.contains(&id) { return Err(format!("Duplicate shard ID {id} in SHARD_IDS.")); }
        ids.push(id);
    }
    Ok(ids)
}
pub fn parse_shard_count(raw: Option<&str>) -> Result<Option<u64>, String> {
    let Some(raw) = raw else { return Ok(None); };
    if raw.trim().is_empty() { return Ok(None); }
    let n = raw.trim().parse::<u64>().map_err(|_| "SHARD_COUNT must be a positive integer when set.".to_string())?;
    if n == 0 { return Err("SHARD_COUNT must be a positive integer when set.".into()); }
    Ok(Some(n))
}
pub fn clean_proxy_url(raw: &str) -> Option<String> {
    let s = raw.trim();
    if s.is_empty() { return None; }
    // Preserve behavior of Python's regex extraction of a URL from a pasted curl command.
    let lower = s.to_ascii_lowercase();
    for scheme in ["https://", "http://"] {
        if let Some(start) = lower.find(scheme) {
            let rest = &s[start..];
            let end = rest.find(|c: char| c.is_whitespace() || c == '"' || c == '\'' || c == '<' || c == '>').unwrap_or(rest.len());
            return Some(rest[..end].trim_end_matches('/').to_string());
        }
    }
    Some(s.to_string())
}
pub fn parse_proxy_pool(raw: &str) -> Vec<String> {
    if ["none","direct","off","disabled","false"].contains(&raw.trim().to_ascii_lowercase().as_str()) { return Vec::new(); }
    let mut urls = Vec::new();
    for part in raw.split(|c| c == ',' || c == ';' || c == '\n' || c == '\r') {
        if let Some(url) = clean_proxy_url(part) {
            if !urls.contains(&url) { urls.push(url); }
        }
    }
    urls
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn invalid_app_id_rejected() { assert_eq!(parse_application_id("12x"), None); assert_eq!(parse_application_id("0"), None); }
    #[test] fn valid_app_id_parsed() { assert_eq!(parse_application_id("123456789"), Some(123456789)); }
    #[test] fn shard_ids_duplicates_rejected() { assert!(parse_shard_ids("1,2,1").is_err()); }
    #[test] fn shard_ids_whitespace_accepted() { assert_eq!(parse_shard_ids(" 2, 5 ").unwrap(), vec![2,5]); }
    #[test] fn zero_shard_count_rejected() { assert!(parse_shard_count(Some("0")).is_err()); }
    #[test] fn proxy_pool_deduplicates() { assert_eq!(parse_proxy_pool("http://x;http://x"), vec!["http://x"]); }
}
