use indexmap::IndexMap;
use nostr_sdk::nostr::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use yew::Reducible;

#[derive(Debug, Clone)]
pub struct UserEventMsg {
    pub event: Event,
    pub visible: bool,
}
#[derive(Debug, Clone, Default)]
pub struct UserEvent {
    pub events: Rc<RefCell<IndexMap<EventId, UserEventMsg>>>,
    count: u64,
}

impl PartialEq for UserEvent {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}
pub enum UserEventAction {
    Add(Event),
    AddVisible(Event),
    Remove(EventId),
    Visible(EventId, bool),
    Update(UserEventMsg),
}
impl Reducible for UserEvent {
    type Action = UserEventAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        use UserEventAction::*;
        let mut count = self.count;
        let evs = self.events.clone();
        match action {
            Add(event) => {
                evs.borrow_mut().insert(
                    event.id,
                    UserEventMsg {
                        event,
                        visible: false,
                    },
                );
            }
            Update(e) => {
                evs.borrow_mut().insert(e.event.id, e);
            }
            Remove(eid) => {
                evs.borrow_mut().remove(&eid);
            }
            Visible(eid, visible) => {
                if let Some(e) = evs.borrow_mut().get_mut(&eid) {
                    e.visible = visible;
                }
            }
            AddVisible(event) => {
                evs.borrow_mut().insert(
                    event.id,
                    UserEventMsg {
                        event,
                        visible: true,
                    },
                );
            }
        };
        count = count.checked_add(1).or(Some(0)).unwrap();
        evs.borrow_mut().sort_by(|_, v1, _, v2| {
            v2.event
                .created_at
                .as_u64()
                .cmp(&v1.event.created_at.as_u64())
        });
        std::rc::Rc::new(UserEvent { events: evs, count })
    }
}
pub type UserEventContext = yew::UseReducerHandle<UserEvent>;
