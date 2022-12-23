/// A stack that only allows a single item to be pushed at a given depth.
#[derive(Debug)]
pub struct DepthStack<T> {
    stack: Vec<(usize, T)>,
    depth: usize,
}

impl<T> DepthStack<T> {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            depth: 0,
        }
    }

    pub fn push(&mut self, item: T) -> Option<T> {
        if self
            .stack
            .last()
            .filter(|(d, _)| *d == self.depth)
            .is_some()
        {
            return self.stack.pop().map(|(_, v)| v);
        }

        self.stack.push((self.depth, item));
        None
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop().map(|(_, item)| item)
    }

    #[allow(dead_code)]
    pub fn peek(&self) -> Option<&T> {
        self.stack
            .last()
            .filter(|(d, _)| *d == self.depth)
            .map(|(_, item)| item)
    }

    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.depth
    }

    #[tracing::instrument(skip(self))]
    pub fn inc_depth(&mut self) {
        self.depth += 1;
        tracing::trace!(depth = self.depth);
    }

    #[tracing::instrument(skip(self))]
    pub fn dec_depth(&mut self) {
        self.depth -= 1;
        tracing::trace!(depth = self.depth);
        self.stack.retain(|(d, _)| *d <= self.depth);
    }
}

impl<T> Default for DepthStack<T> {
    fn default() -> Self {
        Self::new()
    }
}
