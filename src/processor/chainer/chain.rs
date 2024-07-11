use crate::handler::query::{Fit, ProcessParams};
use crate::processor::chain::ProcessorChainBuilder;
use crate::processor::error::Error;
use crate::processor::Image;
use crate::processor::procs::crop::Crop as CropProcessor;
use crate::processor::procs::resize::Resize as ResizeProcessor;
use crate::processor::procs::flip::Flip as FlipProcessor;
use crate::processor::procs::rotate::Rotate as RotateProcessor;
use crate::processor::procs::monochrome::MonoChrome as MonoChromeProcessor;
use crate::processor::procs::blur::Blur as BlurProcessor;

pub struct Processor {}

impl Processor {
    pub fn process(&self, image: &mut Image, params: ProcessParams) -> Result<(), Error> {
        let mut cb = ProcessorChainBuilder::new();

        if let Some(flip) = params.flip {
            cb.add_processor(FlipProcessor { flip_type: flip.into() });
        }

        if let Some(rotate) = params.rotate {
            cb.add_processor(RotateProcessor { degrees: rotate.0 });
        }

        match params.fit {
            Some(Fit::Crop) => {
                cb.add_processor(CropProcessor {
                    width: params.width.unwrap_or(0) as u32,
                    height: params.height.unwrap_or(0) as u32,
                    point: params.crop.into(),
                });
            }
            Some(Fit::Scale) => {
                cb.add_processor(ResizeProcessor {
                    width: params.width.unwrap_or(0) as u32,
                    height: params.height.unwrap_or(0) as u32,
                    maintain_aspect_ratio: false,
                });
            }
            None => {
                if params.width.is_some() || params.height.is_some() {
                    cb.add_processor(ResizeProcessor {
                        width: params.width.unwrap_or(0) as u32,
                        height: params.height.unwrap_or(0) as u32,
                        maintain_aspect_ratio: true,
                    });
                }
            }
        }

        if let Some(monochrome) = params.monochrome {
            cb.add_processor(MonoChromeProcessor { color: monochrome });
        }

        if let Some(blur) = params.blur {
            cb.add_processor(BlurProcessor { radius: blur });
        }

        cb.build().reduce(image)
    }
}
