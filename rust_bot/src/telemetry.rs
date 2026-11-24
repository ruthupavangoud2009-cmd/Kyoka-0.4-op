//! Pure telemetry formatting port from `src/cogs/general/stats.py`.
//! Runtime metrics are supplied by the Serenity/runtime adapter.

#[derive(Debug, Clone, PartialEq)]
pub struct Telemetry {
    pub uptime_seconds: u64,
    pub gateway_latency_ms: u64,
    pub process_ram_mb: f64,
    pub total_system_ram_gb: f64,
    pub cpu_percent: f64,
    pub guild_count: u64,
    pub member_count: u64,
    pub runtime: String,
    pub platform: String,
}

fn grouped(value: u64) -> String {
    let raw = value.to_string();
    let mut out = String::with_capacity(raw.len() + raw.len() / 3);
    for (i, ch) in raw.chars().enumerate() {
        if i > 0 && (raw.len() - i) % 3 == 0 { out.push(','); }
        out.push(ch);
    }
    out
}

impl Telemetry {
    pub fn uptime_string(&self) -> String {
        let days = self.uptime_seconds / 86_400;
        let hours = (self.uptime_seconds / 3_600) % 24;
        let minutes = (self.uptime_seconds / 60) % 60;
        if days > 0 { format!("{days}d {hours}h {minutes}m") }
        else if hours > 0 { format!("{hours}h {minutes}m") }
        else { format!("{minutes}m") }
    }

    pub fn report(&self, bot_name: &str) -> String {
        format!(
            "### {bot_name} Telemetry\n**Process Uptime:** `{}`\n**Gateway Latency:** `{}ms`\n\n**System & Resources**\nRAM Usage: `{:.1} MB` / `{:.1} GB`\nProcess CPU: `{:.1}%`\nRuntime: `{}`\nPlatform: `{}`\n\n**Network Reach**\nGuilds: `{}` servers\nUsers In Scope: `{}` members",
            self.uptime_string(), self.gateway_latency_ms, self.process_ram_mb,
            self.total_system_ram_gb, self.cpu_percent, self.runtime, self.platform,
            grouped(self.guild_count), grouped(self.member_count)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Telemetry {
        Telemetry { uptime_seconds: 60, gateway_latency_ms: 25, process_ram_mb: 64.25,
            total_system_ram_gb: 8.0, cpu_percent: 2.5, guild_count: 1234,
            member_count: 98_765, runtime: "Rust".into(), platform: "Linux".into() }
    }

    #[test]
    fn uptime_units_roll_over() {
        assert_eq!(example().uptime_string(), "1m");
        let mut t = example(); t.uptime_seconds = 3_661;
        assert_eq!(t.uptime_string(), "1h 1m");
        t.uptime_seconds = 90_061;
        assert_eq!(t.uptime_string(), "1d 1h 1m");
    }

    #[test]
    fn report_contains_metrics_and_grouped_counts() {
        let report = example().report("KYOKA");
        assert!(report.contains("KYOKA Telemetry"));
        assert!(report.contains("64.3 MB"));
        assert!(report.contains("1,234"));
        assert!(report.contains("98,765"));
        assert!(report.contains("25ms"));
    }
}
