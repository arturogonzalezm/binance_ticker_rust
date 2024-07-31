use crate::ticker::Ticker;

pub trait Observer {
    fn update(&self, ticker: &Ticker);
}

pub trait Subject {
    fn register_observer(&mut self, observer: Box<dyn Observer + Send + Sync>);
    fn remove_observer(&mut self, observer_id: usize);
    fn notify_observers(&self, ticker: &Ticker);
}

pub struct TickerSubject {
    observers: Vec<(usize, Box<dyn Observer + Send + Sync>)>,
    next_id: usize,
}

impl TickerSubject {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
            next_id: 0,
        }
    }
}

impl Subject for TickerSubject {
    fn register_observer(&mut self, observer: Box<dyn Observer + Send + Sync>) {
        self.observers.push((self.next_id, observer));
        self.next_id += 1;
    }

    fn remove_observer(&mut self, observer_id: usize) {
        self.observers.retain(|(id, _)| *id != observer_id);
    }

    fn notify_observers(&self, ticker: &Ticker) {
        for (_, observer) in &self.observers {
            observer.update(ticker);
        }
    }
}

pub struct PrintObserver;

impl Observer for PrintObserver {
    fn update(&self, ticker: &Ticker) {
        println!("Received ticker update: {:?}", ticker);
    }
}
