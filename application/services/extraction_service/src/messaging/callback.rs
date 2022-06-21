pub struct Processor<T> {
    pub callback: Box<dyn FnMut(T) + Send + Sync + 'static>,
}

impl<T> Processor<T> {
    pub fn process_event(&mut self, event: T) {
        (self.callback)(event);
    }
}
