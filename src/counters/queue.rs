use crate::choices::{ArsenalItem, DequeueChoice};

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Queue {
    entrance: Option<ArsenalItem>,
    pool: Pool,
    exit: Option<ArsenalItem>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            entrance: None,
            pool: Pool {
                items: vec![ArsenalItem::Mirror],
            },
            exit: None,
        }
    }

    pub fn enqueue(&mut self, entering: Option<ArsenalItem>) {
        let entrance = self.entrance.take();
        if let Some(entrance) = entrance {
            self.pool.items.push(entrance);
        }
        self.entrance = entering;
    }

    pub fn dequeue(&mut self, choice: DequeueChoice) -> Result<Option<ArsenalItem>, ()> {
        match choice {
            DequeueChoice::Decline => Ok(None),
            DequeueChoice::JustExit => Ok(self.exit.take()),
            DequeueChoice::DrainAndExit(drainee) => self.drain(drainee),
        }
    }

    fn drain(&mut self, drainee: ArsenalItem) -> Result<Option<ArsenalItem>, ()> {
        let position = self.pool.items.iter().position(|m| m == &drainee);
        match position {
            None => Err(()),
            Some(position) => {
                let drainee = self.pool.items.remove(position);
                let exiting = self.exit.take();
                self.exit = Some(drainee);
                Ok(exiting)
            }
        }
    }

    pub fn pool(&self) -> &Vec<ArsenalItem> {
        &self.pool.items
    }

    pub fn exit_vacant(&self) -> bool {
        match self.exit {
            None => true,
            Some(_) => false,
        }
    }
}

#[derive(Debug, Clone)]
struct Pool {
    items: Vec<ArsenalItem>,
}

impl Pool {
    fn hash_map(&self) -> HashMap<&ArsenalItem, usize> {
        let mut map: HashMap<&ArsenalItem, usize> = HashMap::new();
        for item in self.items.iter() {
            *map.entry(item).or_insert(0) += 1;
        }
        map
    }
}

impl PartialEq for Pool {
    fn eq(&self, other: &Pool) -> bool {
        self.hash_map() == other.hash_map()
    }
}

impl Eq for Pool {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::choices::Move;

    #[test]
    fn enqueue_moves_item_to_entrance() {
        let mut queue = Queue::new();
        queue.enqueue(Some(ArsenalItem::Move(Move::Kick)));
        assert_eq!(queue.entrance, Some(ArsenalItem::Move(Move::Kick)));
    }

    #[test]
    fn enqueue_works_without_entering_item() {
        let mut queue = Queue {
            entrance: Some(ArsenalItem::Move(Move::Kick)),
            pool: Pool { items: vec![] },
            exit: None,
        };
        queue.enqueue(None);
        assert!(queue.pool.items.contains(&ArsenalItem::Move(Move::Kick)));
    }

    #[test]
    fn enqueue_moves_entrance_into_pool() {
        let mut queue = Queue {
            entrance: Some(ArsenalItem::Move(Move::Kick)),
            pool: Pool { items: vec![] },
            exit: None,
        };
        queue.enqueue(Some(ArsenalItem::Move(Move::NinjaSword)));
        assert_eq!(queue.pool.items, vec![ArsenalItem::Move(Move::Kick)]);
    }

    #[test]
    fn dequeue_returns_exit() {
        let mut queue = Queue {
            entrance: Some(ArsenalItem::Move(Move::Kick)),
            pool: Pool {
                items: vec![ArsenalItem::Mirror],
            },
            exit: Some(ArsenalItem::Move(Move::Nunchucks)),
        };
        let returned = queue
            .dequeue(DequeueChoice::DrainAndExit(ArsenalItem::Mirror))
            .unwrap();
        assert_eq!(returned, Some(ArsenalItem::Move(Move::Nunchucks)));
    }

    #[test]
    fn dequeue_moves_drainee_from_pool_to_exit() {
        let mut queue = Queue {
            entrance: Some(ArsenalItem::Move(Move::Kick)),
            pool: Pool {
                items: vec![ArsenalItem::Mirror],
            },
            exit: Some(ArsenalItem::Move(Move::Nunchucks)),
        };
        queue
            .dequeue(DequeueChoice::DrainAndExit(ArsenalItem::Mirror))
            .unwrap();
        assert_eq!(queue.exit, Some(ArsenalItem::Mirror));
        assert!(!queue.pool.items.contains(&ArsenalItem::Mirror));
    }

    #[test]
    fn dequeue_works_without_drainee() {
        let mut queue = Queue {
            entrance: Some(ArsenalItem::Move(Move::Kick)),
            pool: Pool {
                items: vec![ArsenalItem::Mirror],
            },
            exit: Some(ArsenalItem::Move(Move::Nunchucks)),
        };
        assert_eq!(
            queue.dequeue(DequeueChoice::JustExit),
            Ok(Some(ArsenalItem::Move(Move::Nunchucks)))
        );
        assert_eq!(queue.pool.items, vec![ArsenalItem::Mirror]);
    }

    #[test]
    fn exit_vacant_works() {
        let occupied_exit = Queue {
            entrance: Some(ArsenalItem::Move(Move::Kick)),
            pool: Pool {
                items: vec![ArsenalItem::Mirror],
            },
            exit: Some(ArsenalItem::Move(Move::Nunchucks)),
        };
        assert!(!occupied_exit.exit_vacant());

        let occupied_exit = Queue {
            entrance: Some(ArsenalItem::Move(Move::Kick)),
            pool: Pool {
                items: vec![ArsenalItem::Mirror],
            },
            exit: None,
        };
        assert!(occupied_exit.exit_vacant());
    }

    #[test]
    fn queues_are_equal_if_their_entrances_and_exits_are_equal_and_their_pools_have_the_same_items_even_if_they_are_in_different_orders(
    ) {
        let kick = ArsenalItem::Move(Move::Kick);
        let nunchucks = ArsenalItem::Move(Move::Nunchucks);
        let a = Queue {
            entrance: None,
            pool: Pool {
                items: vec![kick, nunchucks],
            },
            exit: None,
        };
        let b = Queue {
            entrance: None,
            pool: Pool {
                items: vec![nunchucks, kick],
            },
            exit: None,
        };
        assert!(a == b);
        assert!(b == a);
    }

    #[test]
    fn queues_are_not_equal_if_they_have_different_entrances() {
        let kick = ArsenalItem::Move(Move::Kick);
        let nunchucks = ArsenalItem::Move(Move::Nunchucks);
        let fireball = ArsenalItem::Move(Move::ShadowFireball);
        let a = Queue {
            entrance: Some(fireball),
            pool: Pool {
                items: vec![kick, nunchucks],
            },
            exit: None,
        };
        let b = Queue {
            entrance: None,
            pool: Pool {
                items: vec![nunchucks, kick],
            },
            exit: None,
        };
        assert!(a != b);
        assert!(b != a);
    }

    #[test]
    fn queues_are_not_equal_if_they_have_different_exits() {
        let kick = ArsenalItem::Move(Move::Kick);
        let nunchucks = ArsenalItem::Move(Move::Nunchucks);
        let fireball = ArsenalItem::Move(Move::ShadowFireball);
        let a = Queue {
            entrance: None,
            pool: Pool {
                items: vec![kick, nunchucks],
            },
            exit: Some(fireball),
        };
        let b = Queue {
            entrance: None,
            pool: Pool {
                items: vec![nunchucks, kick],
            },
            exit: None,
        };
        assert!(a != b);
        assert!(b != a);
    }

    #[test]
    fn queues_are_not_equal_if_they_have_different_amounts_of_each_pool_item() {
        let kick = ArsenalItem::Move(Move::Kick);
        let nunchucks = ArsenalItem::Move(Move::Nunchucks);
        let a = Queue {
            entrance: None,
            pool: Pool {
                items: vec![kick, nunchucks],
            },
            exit: None,
        };
        let b = Queue {
            entrance: None,
            pool: Pool {
                items: vec![nunchucks, kick, kick],
            },
            exit: None,
        };
        assert!(a != b);
        assert!(b != a);
    }
}
