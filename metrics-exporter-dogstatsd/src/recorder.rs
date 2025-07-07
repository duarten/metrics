use std::sync::Arc;

use metrics::{Counter, Gauge, Histogram, Key, KeyName, Metadata, Recorder, SharedString, Unit};

use crate::{state::State, AtomicCounter, AtomicGauge, AtomicHistogram};

/// A recorder that forwards metrics to a DogStatsD server.
pub struct DogStatsDRecorder {
    state: Arc<State>,
}

impl DogStatsDRecorder {
    pub(crate) fn new(state: Arc<State>) -> Self {
        DogStatsDRecorder { state }
    }

    /// Registers a new counter with the given key and metadata.
    pub fn register_counter(&self, key: &Key, _: &Metadata<'_>) -> Arc<AtomicCounter> {
        self.state.registry().get_or_create_counter(key, Arc::clone)
    }

    /// Registers a new gauge with the given key and metadata.
    pub fn register_gauge(&self, key: &Key, _: &Metadata<'_>) -> Arc<AtomicGauge> {
        self.state.registry().get_or_create_gauge(key, Arc::clone)
    }

    /// Registers a new histogram with the given key and metadata.
    pub fn register_histogram(&self, key: &Key, _: &Metadata<'_>) -> Arc<AtomicHistogram> {
        self.state.registry().get_or_create_histogram(key, Arc::clone)
    }
}

impl Recorder for DogStatsDRecorder {
    fn describe_counter(&self, _: KeyName, _: Option<Unit>, _: SharedString) {}
    fn describe_gauge(&self, _: KeyName, _: Option<Unit>, _: SharedString) {}
    fn describe_histogram(&self, _: KeyName, _: Option<Unit>, _: SharedString) {}

    fn register_counter(&self, key: &Key, metadata: &Metadata<'_>) -> Counter {
        Counter::from_arc(self.register_counter(key, metadata))
    }

    fn register_gauge(&self, key: &Key, metadata: &Metadata<'_>) -> Gauge {
        Gauge::from_arc(self.register_gauge(key, metadata))
    }

    fn register_histogram(&self, key: &Key, metadata: &Metadata<'_>) -> Histogram {
        Histogram::from_arc(self.register_histogram(key, metadata))
    }
}
