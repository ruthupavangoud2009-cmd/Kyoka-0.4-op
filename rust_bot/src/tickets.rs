use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TicketStatus { Open, Claimed, Closed }
#[derive(Clone, Debug)]
pub struct Ticket {
 pub id: u64, pub guild_id: u64, pub opener_id: u64, pub channel_id: u64,
 pub claimed_by: Option<u64>, pub status: TicketStatus, pub close_reason: Option<String>,
}
#[derive(Default)]
pub struct TicketRegistry { tickets: HashMap<u64, Ticket>, next_id: u64 }
impl TicketRegistry {
 pub fn create(&mut self, guild_id:u64, opener_id:u64, channel_id:u64)->Ticket {
  self.next_id+=1; let ticket=Ticket{id:self.next_id,guild_id,opener_id,channel_id,claimed_by:None,status:TicketStatus::Open,close_reason:None};
  self.tickets.insert(ticket.id,ticket.clone()); ticket
 }
 pub fn claim(&mut self,id:u64,staff_id:u64)->Result<&Ticket,&'static str>{
  let t=self.tickets.get_mut(&id).ok_or("ticket not found")?;
  if t.status!=TicketStatus::Open{return Err("ticket is not open")}
  t.claimed_by=Some(staff_id);t.status=TicketStatus::Claimed;Ok(t)
 }
 pub fn close(&mut self,id:u64,reason:Option<String>)->Result<&Ticket,&'static str>{
  let t=self.tickets.get_mut(&id).ok_or("ticket not found")?;
  if t.status==TicketStatus::Closed{return Err("ticket already closed")}
  t.status=TicketStatus::Closed;t.close_reason=reason;Ok(t)
 }
 pub fn get(&self,id:u64)->Option<&Ticket>{self.tickets.get(&id)}
}
#[cfg(test)]
mod tests {
 use super::*;
 #[test] fn create_claim_close_lifecycle(){
  let mut reg=TicketRegistry::default();let t=reg.create(1,2,3);assert_eq!(t.status,TicketStatus::Open);
  assert_eq!(reg.claim(t.id,9).unwrap().status,TicketStatus::Claimed);
  assert_eq!(reg.close(t.id,Some("resolved".into())).unwrap().status,TicketStatus::Closed);
 }
 #[test] fn rejects_claiming_twice(){let mut reg=TicketRegistry::default();let t=reg.create(1,2,3);reg.claim(t.id,9).unwrap();assert!(reg.claim(t.id,10).is_err());}
}
