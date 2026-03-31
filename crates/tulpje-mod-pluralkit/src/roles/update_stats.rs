#[derive(Debug, Clone)]
pub(crate) struct UpdateCounts {
    pub(crate) create: u16,
    pub(crate) update: u16,
    pub(crate) delete: u16,
    pub(crate) assign: u16,
}

impl UpdateCounts {
    fn new() -> Self {
        Self {
            create: 0,
            update: 0,
            delete: 0,
            assign: 0,
        }
    }

    fn with_counts(create: u16, update: u16, delete: u16, assign: u16) -> Self {
        Self {
            create,
            update,
            delete,
            assign,
        }
    }

    pub(super) fn sum(&self) -> u16 {
        self.create + self.update + self.delete + self.assign
    }
}

#[derive(Debug, Clone)]
pub(crate) struct UpdateStats {
    pub(crate) done: UpdateCounts,
    pub(crate) total: UpdateCounts,
}

impl UpdateStats {
    pub(crate) fn new(create: u16, update: u16, delete: u16, assign: u16) -> Self {
        Self {
            done: UpdateCounts::new(),
            total: UpdateCounts::with_counts(create, update, delete, assign),
        }
    }
}
