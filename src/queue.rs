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
