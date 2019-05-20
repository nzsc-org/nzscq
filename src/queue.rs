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

    pub fn enqueue(&mut self, entering: ArsenalItem) {
        let entrance = self.entrance.take();
        if let Some(entrance) = entrance {
            self.pool.push(entrance);
        }
        self.entrance = Some(entering);
    }

    pub fn dequeue(&mut self, drainee: &ArsenalItem) -> Option<ArsenalItem> {
        let position = self
            .pool
            .iter()
            .position(|m| m == drainee)
            .expect("The move you chose to drain is not in the pool.");
        let drainee = self.pool.remove(position);
        let exiting = self.exit.take();
        self.exit = Some(drainee);
        exiting
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
        queue.enqueue(ArsenalItem::Move(Move::Kick));
        assert_eq!(queue.entrance, Some(ArsenalItem::Move(Move::Kick)));
    }

    #[test]
    fn enqueue_moves_entrance_into_pool() {
        let mut queue = Queue {
            entrance: Some(ArsenalItem::Move(Move::Kick)),
            pool: vec![],
            exit: None,
        };
        queue.enqueue(ArsenalItem::Move(Move::NinjaSword));
        assert_eq!(queue.pool, vec![ArsenalItem::Move(Move::Kick)]);
    }

    #[test]
    fn dequeue_returns_exit() {
        let mut queue = Queue {
            entrance: Some(ArsenalItem::Move(Move::Kick)),
            pool: vec![ArsenalItem::Mirror],
            exit: Some(ArsenalItem::Move(Move::Nunchucks)),
        };
        let returned = queue.dequeue(&ArsenalItem::Mirror);
        assert_eq!(returned, Some(ArsenalItem::Move(Move::Nunchucks)));
    }

    #[test]
    fn dequeue_moves_drainee_from_pool_to_exit() {
        let mut queue = Queue {
            entrance: Some(ArsenalItem::Move(Move::Kick)),
            pool: vec![ArsenalItem::Mirror],
            exit: Some(ArsenalItem::Move(Move::Nunchucks)),
        };
        queue.dequeue(&ArsenalItem::Mirror);
        assert_eq!(queue.exit, Some(ArsenalItem::Mirror));
        assert!(!queue.pool.contains(&ArsenalItem::Mirror));
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
