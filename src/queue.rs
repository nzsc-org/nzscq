use crate::player::ArsenalItem;

#[derive(Debug, Clone)]
pub struct Queue {
    entrance: Option<ArsenalItem>,
    pool: Vec<ArsenalItem>,
    exit: Option<ArsenalItem>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            entrance: None,
            pool: vec![ArsenalItem::Mirror],
            exit: None,
        }
    }

    pub fn enqueue(&mut self, entering: Option<ArsenalItem>) {
        let entrance = self.entrance.take();
        if let Some(entrance) = entrance {
            self.pool.push(entrance);
        }
        self.entrance = entering;
    }

    pub fn dequeue(&mut self, drainee: Option<&ArsenalItem>) -> Result<Option<ArsenalItem>, ()> {
        match drainee {
            None => Ok(self.exit.take()),
            Some(drainee) => self.drain(drainee),
        }
    }

    fn drain(&mut self, drainee: &ArsenalItem) -> Result<Option<ArsenalItem>, ()> {
        let position = self.pool.iter().position(|m| m == drainee);
        match position {
            None => Err(()),
            Some(position) => {
                let drainee = self.pool.remove(position);
                let exiting = self.exit.take();
                self.exit = Some(drainee);
                Ok(exiting)
            }
        }
    }

    pub fn pool(&self) -> &Vec<ArsenalItem> {
        &self.pool
    }

    pub fn exit_vacant(&self) -> bool {
        match self.exit {
            None => true,
            Some(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::moves::Move;

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
            pool: vec![],
            exit: None,
        };
        queue.enqueue(None);
        assert!(queue.pool.contains(&ArsenalItem::Move(Move::Kick)));
    }

    #[test]
    fn enqueue_moves_entrance_into_pool() {
        let mut queue = Queue {
            entrance: Some(ArsenalItem::Move(Move::Kick)),
            pool: vec![],
            exit: None,
        };
        queue.enqueue(Some(ArsenalItem::Move(Move::NinjaSword)));
        assert_eq!(queue.pool, vec![ArsenalItem::Move(Move::Kick)]);
    }

    #[test]
    fn dequeue_returns_exit() {
        let mut queue = Queue {
            entrance: Some(ArsenalItem::Move(Move::Kick)),
            pool: vec![ArsenalItem::Mirror],
            exit: Some(ArsenalItem::Move(Move::Nunchucks)),
        };
        let returned = queue.dequeue(Some(&ArsenalItem::Mirror)).unwrap();
        assert_eq!(returned, Some(ArsenalItem::Move(Move::Nunchucks)));
    }

    #[test]
    fn dequeue_moves_drainee_from_pool_to_exit() {
        let mut queue = Queue {
            entrance: Some(ArsenalItem::Move(Move::Kick)),
            pool: vec![ArsenalItem::Mirror],
            exit: Some(ArsenalItem::Move(Move::Nunchucks)),
        };
        queue.dequeue(Some(&ArsenalItem::Mirror)).unwrap();
        assert_eq!(queue.exit, Some(ArsenalItem::Mirror));
        assert!(!queue.pool.contains(&ArsenalItem::Mirror));
    }

    #[test]
    fn dequeue_works_without_drainee() {
        let mut queue = Queue {
            entrance: Some(ArsenalItem::Move(Move::Kick)),
            pool: vec![ArsenalItem::Mirror],
            exit: Some(ArsenalItem::Move(Move::Nunchucks)),
        };
        assert_eq!(
            queue.dequeue(None),
            Ok(Some(ArsenalItem::Move(Move::Nunchucks)))
        );
        assert_eq!(queue.pool, vec![ArsenalItem::Mirror]);
    }

    #[test]
    fn exit_vacant_works() {
        let occupied_exit = Queue {
            entrance: Some(ArsenalItem::Move(Move::Kick)),
            pool: vec![ArsenalItem::Mirror],
            exit: Some(ArsenalItem::Move(Move::Nunchucks)),
        };
        assert!(!occupied_exit.exit_vacant());

        let occupied_exit = Queue {
            entrance: Some(ArsenalItem::Move(Move::Kick)),
            pool: vec![ArsenalItem::Mirror],
            exit: None,
        };
        assert!(occupied_exit.exit_vacant());
    }
}
