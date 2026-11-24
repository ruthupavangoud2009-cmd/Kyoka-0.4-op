//! Core member-count calculation and response formatting ported from
//! `src/cogs/general/membercount.py`. Discord data acquisition remains in the
//! gateway adapter layer.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemberCounts {
    pub total: u64,
    pub bots: u64,
    pub humans: u64,
}

impl MemberCounts {
    /// Compute counts from the guild total and the bot members available in cache.
    /// Saturating subtraction avoids negative human counts if the cache is partial.
    pub fn from_total_and_cached_bots(total: u64, cached_bots: u64) -> Self {
        Self {
            total,
            bots: cached_bots.min(total),
            humans: total.saturating_sub(cached_bots),
        }
    }

    /// Compute counts when the full member list is available.
    pub fn from_members(is_bot: impl IntoIterator<Item = bool>) -> Self {
        let (total, bots) = is_bot.into_iter().fold((0_u64, 0_u64), |(total, bots), bot| {
            (total + 1, bots + u64::from(bot))
        });
        Self { total, bots, humans: total - bots }
    }

    fn grouped_number(value: u64) -> String {
        let raw = value.to_string();
        let mut out = String::with_capacity(raw.len() + raw.len() / 3);
        for (i, ch) in raw.chars().enumerate() {
            if i > 0 && (raw.len() - i) % 3 == 0 { out.push(','); }
            out.push(ch);
        }
        out
    }

    pub fn response_lines(&self, dot: &str) -> String {
        let prefix = if dot.is_empty() { String::new() } else { format!("{dot} ") };
        format!(
            "{prefix}**Total Member:** `{}`\n{prefix}**Humans:** `{}`\n{prefix}**Bot:** `{}`",
            Self::grouped_number(self.total), Self::grouped_number(self.humans), Self::grouped_number(self.bots)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_from_full_member_list() {
        let counts = MemberCounts::from_members([false, true, false, true, false]);
        assert_eq!(counts, MemberCounts { total: 5, bots: 2, humans: 3 });
    }

    #[test]
    fn cached_bot_count_is_clamped_to_total() {
        let counts = MemberCounts::from_total_and_cached_bots(4, 8);
        assert_eq!(counts, MemberCounts { total: 4, bots: 4, humans: 0 });
    }

    #[test]
    fn formats_count_lines_with_optional_dot() {
        let counts = MemberCounts { total: 1234, bots: 12, humans: 1222 };
        assert_eq!(
            counts.response_lines("♥"),
            "♥ **Total Member:** `1,234`\n♥ **Humans:** `1,222`\n♥ **Bot:** `12`"
        );
        assert_eq!(counts.response_lines(""), "**Total Member:** `1,234`\n**Humans:** `1,222`\n**Bot:** `12`");
    }
}
