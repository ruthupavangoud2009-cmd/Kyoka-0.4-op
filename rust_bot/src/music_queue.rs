//! Guild-local music queue state. This module is transport-independent so Lavalink/Discord
//! adapters can use it without sharing mutable queue state between guilds.

use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Track {
    pub title: String,
    pub uri: String,
    pub duration_ms: Option<u64>,
    pub requester_id: u64,
}

impl Track {
    pub fn new(title: impl Into<String>, uri: impl Into<String>, duration_ms: Option<u64>, requester_id: u64) -> Self {
        Self { title: title.into(), uri: uri.into(), duration_ms, requester_id }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RepeatMode { Off, Track, Queue }

#[derive(Clone, Debug)]
pub struct MusicQueue {
    upcoming: VecDeque<Track>,
    current: Option<Track>,
    previous: Vec<Track>,
    repeat: RepeatMode,
    shuffle: bool,
    shuffle_seed: u64,
}

impl Default for MusicQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl MusicQueue {
    pub fn new() -> Self {
        Self { upcoming: VecDeque::new(), current: None, previous: Vec::new(), repeat: RepeatMode::Off, shuffle: false, shuffle_seed: 0x9E3779B97F4A7C15 }
    }
    pub fn len(&self) -> usize { self.upcoming.len() }
    pub fn is_empty(&self) -> bool { self.upcoming.is_empty() && self.current.is_none() }
    pub fn current(&self) -> Option<&Track> { self.current.as_ref() }
    pub fn upcoming(&self) -> &VecDeque<Track> { &self.upcoming }
    pub fn repeat_mode(&self) -> RepeatMode { self.repeat }
    pub fn shuffle_enabled(&self) -> bool { self.shuffle }
    pub fn enqueue(&mut self, track: Track) { self.upcoming.push_back(track); }
    pub fn enqueue_many<I: IntoIterator<Item = Track>>(&mut self, tracks: I) { self.upcoming.extend(tracks); }
    pub fn set_repeat(&mut self, mode: RepeatMode) { self.repeat = mode; }
    pub fn toggle_shuffle(&mut self) -> bool {
        self.shuffle = !self.shuffle;
        if self.shuffle { self.shuffle_upcoming(); }
        self.shuffle
    }
    pub fn shuffle_upcoming(&mut self) {
        let mut items: Vec<_> = self.upcoming.drain(..).collect();
        // Deterministic Fisher-Yates shuffle. A changing seed keeps this dependency-free.
        for i in (1..items.len()).rev() {
            self.shuffle_seed ^= self.shuffle_seed << 13;
            self.shuffle_seed ^= self.shuffle_seed >> 7;
            self.shuffle_seed ^= self.shuffle_seed << 17;
            let j = (self.shuffle_seed as usize) % (i + 1);
            items.swap(i, j);
        }
        self.upcoming.extend(items);
    }
    /// Selects the next track. Track-repeat replays current; queue-repeat appends it.
    pub fn advance(&mut self) -> Option<Track> {
        if self.repeat == RepeatMode::Track {
            if let Some(track) = self.current.clone() { return Some(track); }
        }
        if let Some(old) = self.current.take() {
            if self.repeat == RepeatMode::Queue { self.upcoming.push_back(old.clone()); }
            self.previous.push(old);
        }
        self.current = self.upcoming.pop_front();
        self.current.clone()
    }
    /// Moves back one track from history, putting the current track at the front.
    pub fn previous(&mut self) -> Option<Track> {
        let prev = self.previous.pop()?;
        if let Some(cur) = self.current.take() { self.upcoming.push_front(cur); }
        self.current = Some(prev.clone());
        Some(prev)
    }
    pub fn skip(&mut self, count: usize) -> Option<Track> {
        if count == 0 { return self.current.clone(); }
        for _ in 0..count.saturating_sub(1) {
            if let Some(cur) = self.current.take() { self.previous.push(cur); }
            self.current = self.upcoming.pop_front();
            if self.current.is_none() { return None; }
        }
        self.advance()
    }
    pub fn clear_upcoming(&mut self) { self.upcoming.clear(); }
    pub fn stop(&mut self) {
        self.current = None;
        self.upcoming.clear();
        self.previous.clear();
        self.repeat = RepeatMode::Off;
    }
    pub fn remove(&mut self, index: usize) -> Option<Track> { self.upcoming.remove(index) }
    pub fn move_track(&mut self, from: usize, to: usize) -> bool {
        if from >= self.upcoming.len() || to >= self.upcoming.len() || from == to { return false; }
        if let Some(track) = self.upcoming.remove(from) { self.upcoming.insert(to, track); true } else { false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn t(n: &str) -> Track { Track::new(n, format!("https://example.test/{n}"), Some(1000), 7) }
    #[test] fn starts_first_and_advances() { let mut q=MusicQueue::new(); q.enqueue_many([t("a"),t("b")]); assert_eq!(q.advance().unwrap().title,"a"); assert_eq!(q.advance().unwrap().title,"b"); }
    #[test] fn repeat_track_replays_current() { let mut q=MusicQueue::new(); q.enqueue(t("a")); q.advance(); q.set_repeat(RepeatMode::Track); assert_eq!(q.advance().unwrap().title,"a"); assert_eq!(q.len(),0); }
    #[test] fn repeat_queue_recycles_track() { let mut q=MusicQueue::new(); q.enqueue(t("a")); q.enqueue(t("b")); q.set_repeat(RepeatMode::Queue); q.advance(); q.advance(); assert_eq!(q.advance().unwrap().title,"a"); }
    #[test] fn previous_restores_track() { let mut q=MusicQueue::new(); q.enqueue_many([t("a"),t("b")]); q.advance(); q.advance(); assert_eq!(q.previous().unwrap().title,"a"); assert_eq!(q.upcoming().front().unwrap().title,"b"); }
    #[test] fn remove_and_move_validate_indices() { let mut q=MusicQueue::new(); q.enqueue_many([t("a"),t("b"),t("c")]); assert!(q.move_track(0,2)); assert_eq!(q.upcoming()[2].title,"a"); assert_eq!(q.remove(0).unwrap().title,"b"); assert!(q.remove(99).is_none()); }
    #[test] fn skip_multiple() { let mut q=MusicQueue::new(); q.enqueue_many([t("a"),t("b"),t("c")]); q.advance(); assert_eq!(q.skip(2).unwrap().title,"c"); }
    #[test] fn stop_clears_state() { let mut q=MusicQueue::new(); q.enqueue(t("a")); q.advance(); q.stop(); assert!(q.is_empty()); assert_eq!(q.repeat_mode(),RepeatMode::Off); }
}
