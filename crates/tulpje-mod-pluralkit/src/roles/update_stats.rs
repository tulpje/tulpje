#[derive(Debug, Clone)]
pub(crate) struct UpdateProgress {
    pub(crate) done: u16,
    pub(crate) total: u16,
}

impl UpdateProgress {
    fn new(total: u16) -> Self {
        Self { done: 0, total }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct UpdateStats {
    // (done, total)
    pub(crate) create: UpdateProgress,
    pub(crate) update: UpdateProgress,
    pub(crate) delete: UpdateProgress,
    pub(crate) assign: UpdateProgress,
}

impl UpdateStats {
    pub(crate) fn new(create: u16, update: u16, delete: u16, assign: u16) -> Self {
        Self {
            create: UpdateProgress::new(create),
            update: UpdateProgress::new(update),
            delete: UpdateProgress::new(delete),
            assign: UpdateProgress::new(assign),
        }
    }

    pub(crate) fn total(&self) -> UpdateProgress {
        UpdateProgress {
            done: self.create.done + self.update.done + self.delete.done + self.assign.done,
            total: self.create.total + self.update.total + self.delete.total + self.assign.total,
        }
    }
}
