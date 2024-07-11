use crate::processor::error::Error;
use crate::processor::image::Image;
use crate::processor::Processor;

pub struct ProcessorChainBuilder<'a> {
    processors: Vec<Box<dyn Processor + Send + Sync + 'a>>,
}

impl<'a> ProcessorChainBuilder<'a> {
    pub fn new() -> Self { Self { processors: Vec::new() } }

    pub fn add_processor<P>(&mut self, processor: P)
    where
        P: Processor + Send + Sync + 'a,
    {
        self.processors.push(Box::new(processor));
    }

    pub fn build(self) -> ProcessorChain<'a> { ProcessorChain::new(self.processors) }
}

pub struct ProcessorChain<'a> {
    processors: Vec<Box<dyn Processor + Send + Sync + 'a>>,
}

impl<'a> ProcessorChain<'a> {
    fn new(processors: Vec<Box<dyn Processor + Send + Sync + 'a>>) -> Self { Self { processors } }

    pub fn reduce(&self, image: &mut Image) -> Result<(), Error> {
        for processor in &self.processors {
            processor.process(image)?;
        }
        Ok(())
    }
}
