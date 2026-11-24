use std::{collections::VecDeque, time::{Duration, Instant}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionKind { ChannelDelete, RoleDelete, Ban, Kick, WebhookCreate }

#[derive(Clone, Debug)]
pub struct AntiNukeConfig {
    pub window: Duration,
    pub channel_delete_limit: usize,
    pub role_delete_limit: usize,
    pub ban_limit: usize,
    pub kick_limit: usize,
    pub webhook_create_limit: usize,
}
impl Default for AntiNukeConfig {
    fn default() -> Self { Self { window: Duration::from_secs(10), channel_delete_limit: 3, role_delete_limit: 3, ban_limit: 5, kick_limit: 7, webhook_create_limit: 3 } }
}
#[derive(Clone, Debug)]
pub struct ThresholdHit { pub actor_id: u64, pub action: ActionKind, pub count: usize, pub limit: usize }
#[derive(Default)]
pub struct AntiNukeTracker { events: VecDeque<(Instant, u64, ActionKind)> }
impl AntiNukeTracker {
    pub fn record(&mut self, config: &AntiNukeConfig, actor_id: u64, action: ActionKind, now: Instant) -> Option<ThresholdHit> {
        while self.events.front().is_some_and(|(t, _, _)| now.saturating_duration_since(*t) > config.window) { self.events.pop_front(); }
        self.events.push_back((now, actor_id, action));
        let count = self.events.iter().filter(|(_, a, k)| *a == actor_id && *k == action).count();
        let limit = match action { ActionKind::ChannelDelete => config.channel_delete_limit, ActionKind::RoleDelete => config.role_delete_limit, ActionKind::Ban => config.ban_limit, ActionKind::Kick => config.kick_limit, ActionKind::WebhookCreate => config.webhook_create_limit };
        (count >= limit).then_some(ThresholdHit { actor_id, action, count, limit })
    }
}
#[cfg(test)]
mod tests {
 use super::*;
 #[test] fn threshold_is_detected_inside_window() {
  let mut tracker=AntiNukeTracker::default(); let cfg=AntiNukeConfig::default(); let now=Instant::now();
  assert!(tracker.record(&cfg,42,ActionKind::ChannelDelete,now).is_none());
  assert!(tracker.record(&cfg,42,ActionKind::ChannelDelete,now+Duration::from_secs(1)).is_none());
  assert!(tracker.record(&cfg,42,ActionKind::ChannelDelete,now+Duration::from_secs(2)).is_some());
 }
 #[test] fn actors_are_counted_separately() {
  let mut tracker=AntiNukeTracker::default(); let cfg=AntiNukeConfig::default(); let now=Instant::now();
  tracker.record(&cfg,1,ActionKind::Ban,now); tracker.record(&cfg,2,ActionKind::Ban,now);
  assert!(tracker.record(&cfg,1,ActionKind::Ban,now+Duration::from_millis(1)).is_none());
 }
}
