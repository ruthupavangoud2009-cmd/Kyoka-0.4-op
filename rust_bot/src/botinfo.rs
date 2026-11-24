//! Pure runtime statistics and text formatting port of the informational portion of
//! `src/cogs/general/botinfo.py`. Discord-specific data collection and link
//! buttons belong in the Serenity command adapter.

fn grouped_number(value: u64) -> String {
    let raw = value.to_string();
    let mut out = String::with_capacity(raw.len() + raw.len() / 3);
    for (i, ch) in raw.chars().enumerate() {
        if i > 0 && (raw.len() - i) % 3 == 0 { out.push(','); }
        out.push(ch);
    }
    out
}

pub struct BotInfo {
    pub name: String,
    pub owner_id: u64,
    pub bot_id: Option<u64>,
    pub uptime_seconds: u64,
    pub latency_ms: Option<u64>,
    pub guild_count: u64,
    pub member_count: u64,
    pub prefix: String,
}

impl BotInfo {
    pub fn uptime_string(&self) -> String {
        let days = self.uptime_seconds / 86_400;
        let hours = (self.uptime_seconds % 86_400) / 3_600;
        let minutes = (self.uptime_seconds % 3_600) / 60;
        if days > 0 {
            format!("{days}d {hours}h {minutes}m")
        } else {
            format!("{hours}h {minutes}m")
        }
    }

    pub fn summary(&self) -> String {
        let ping = self.latency_ms.map(|v| format!("{v} ms")).unwrap_or_else(|| "Unavailable".to_owned());
        let bot_id = self.bot_id.map(|id| id.to_string()).unwrap_or_else(|| "Unavailable".to_owned());
        format!(
            "**{} Information**\n**Creator:** Aizen\n**Owner ID:** `{}`\n**Bot ID:** `{}`\n\n**Status**\n**Uptime:** `{}`\n**API latency:** `{}` • **Gateway ping:** `{}`\n**Servers:** `{}` • **Members across servers:** `{}`\n\n**Commands**\n**Prefix:** `{}` • **Help:** `{}help` or `/help`",
            self.name, self.owner_id, bot_id, self.uptime_string(), ping, ping, self.guild_count, grouped_number(self.member_count), self.prefix, self.prefix
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> BotInfo {
        BotInfo {
            name: "KYOKA".into(), owner_id: 1505515188991033409,
            bot_id: Some(123456), uptime_seconds: 90_061, latency_ms: Some(42),
            guild_count: 3, member_count: 12_345, prefix: "?".into(),
        }
    }

    #[test]
    fn uptime_formats_days_and_minutes() {
        assert_eq!(sample().uptime_string(), "1d 1h 1m");
    }

    #[test]
    fn summary_includes_runtime_details() {
        let summary = sample().summary();
        assert!(summary.contains("KYOKA Information"));
        assert!(summary.contains("42 ms"));
        assert!(summary.contains("12,345"));
        assert!(summary.contains("?help"));
    }

    #[test]
    fn unavailable_latency_and_bot_id_are_safe() {
        let mut info = sample();
        info.latency_ms = None;
        info.bot_id = None;
        let summary = info.summary();
        assert!(summary.contains("Unavailable"));
        assert!(summary.contains("API latency:** `Unavailable`"));
    }
}
