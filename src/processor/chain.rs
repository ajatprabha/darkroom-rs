use std::sync::Arc;
use std::time;
use opentelemetry::KeyValue;
use crate::processor::error::Error;
use crate::processor::image::Image;
use crate::processor::Processor;
use opentelemetry::metrics::Histogram;

pub struct ProcessorChainBuilder<'a> {
    processors: Vec<Box<dyn Processor + Send + Sync + 'a>>,
    histogram: Arc<Histogram<f64>>,
}

impl<'a> ProcessorChainBuilder<'a> {
    pub fn new(histogram: Arc<Histogram<f64>>) -> Self {
        Self {
            processors: Vec::new(),
            histogram,
        }
    }

    pub fn add_processor<P>(&mut self, processor: P)
    where
        P: Processor + Send + Sync + 'a,
    {
        self.processors.push(Box::new(processor));
    }

    pub fn build(self) -> ProcessorChain<'a> {
        ProcessorChain {
            processors: self.processors,
            histogram: self.histogram,
        }
    }
}

pub struct ProcessorChain<'a> {
    processors: Vec<Box<dyn Processor + Send + Sync + 'a>>,
    histogram: Arc<Histogram<f64>>,
}

impl<'a> ProcessorChain<'a> {
    pub fn reduce(&self, image: &mut Image) -> Result<(), Error> {
        for processor in &self.processors {
            let start = time::Instant::now();

            processor.process(image)?;

            self
                .histogram
                .record(
                    start.elapsed().as_secs_f64(),
                    &[KeyValue::new("processor", processor.name())],
                );
        }
        Ok(())
    }
}
