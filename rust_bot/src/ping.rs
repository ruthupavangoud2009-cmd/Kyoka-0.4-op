//! Port of KYOKA's repeated-ping argument validation from src/cogs/general/ping.py.
use regex::Regex;

pub fn parse_repeated_ping(input: &str) -> Result<(u64, u8), String> {
    let re = Regex::new(r"<@!?([0-9]{15,20})>|(?<![0-9])([0-9]{15,20})(?![0-9])").expect("valid regex");
    let caps = re.captures(input).ok_or_else(|| "Include one user mention and a count from 1 to 5.".to_string())?;
    let target = caps.get(1).or_else(|| caps.get(2)).unwrap().as_str().parse::<u64>()
        .map_err(|_| "Invalid user ID.".to_string())?;
    let whole = caps.get(0).unwrap();
    let count_text = format!("{}{}", &input[..whole.start()], &input[whole.end()..]).trim().to_string();
    if !count_text.chars().all(|c| c.is_ascii_digit()) {
        return Err("Use `&ping @user 3` or `&ping 3 @user`.".into());
    }
    if count_text.len() > 5 || count_text.parse::<u64>().unwrap_or(6) > 5 {
        return Err("Repeated-user pings are capped at 5 per command.".into());
    }
    let count = count_text.parse::<u8>().map_err(|_| "Count must be at least 1.".to_string())?;
    if count < 1 { return Err("Count must be at least 1.".into()); }
    Ok((target, count))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn parses_mention_then_count() { assert_eq!(parse_repeated_ping("<@123456789012345> 3").unwrap(), (123456789012345,3)); }
    #[test] fn parses_count_then_id() { assert_eq!(parse_repeated_ping("2 123456789012345").unwrap(), (123456789012345,2)); }
    #[test] fn rejects_over_limit() { assert!(parse_repeated_ping("<@123456789012345> 6").is_err()); }
    #[test] fn rejects_zero() { assert!(parse_repeated_ping("<@123456789012345> 0").is_err()); }
}
