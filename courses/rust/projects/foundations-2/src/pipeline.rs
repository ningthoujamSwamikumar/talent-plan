use crate::error::PipelineError;
use crate::transforms::Transform;

// ---------------------------------------------------------------------------
// Pipeline
// ---------------------------------------------------------------------------

/// A chain of transforms that are applied in order.
///
/// The pipeline stores transforms as trait objects with a unified error type
/// (`PipelineError`). This allows different concrete transform types to coexist
/// in the same `Vec`.
pub struct Pipeline {
    transforms: Vec<Box<dyn Transform<Error = PipelineError>>>,
}

impl Pipeline {
    /// Create a new, empty pipeline.
    pub fn new() -> Self {
        todo!("Implement Pipeline::new")
    }

    /// Append a transform to the end of the pipeline.
    ///
    /// The transform is boxed and stored as a trait object.
    pub fn add_transform<T>(&mut self, transform: T)
    where
        T: Transform<Error = PipelineError> + 'static,
    {
        todo!("Implement Pipeline::add_transform")
    }

    /// Run the pipeline: feed `input` through each transform in order.
    ///
    /// The output of each transform becomes the input of the next. Returns the
    /// final output or the first error encountered.
    pub fn run(&self, input: &str) -> Result<String, PipelineError> {
        todo!("Implement Pipeline::run")
    }

    /// Return the number of transforms in the pipeline.
    pub fn len(&self) -> usize {
        todo!("Implement Pipeline::len")
    }

    /// Return `true` if the pipeline contains no transforms.
    pub fn is_empty(&self) -> bool {
        todo!("Implement Pipeline::is_empty")
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// PipelineBuilder
// ---------------------------------------------------------------------------

/// A builder for constructing a [`Pipeline`] with a fluent API.
///
/// # Example (once implemented)
///
/// ```ignore
/// let pipeline = PipelineBuilder::new()
///     .add(Uppercase)
///     .add(TrimWhitespace)
///     .build();
/// ```
pub struct PipelineBuilder {
    transforms: Vec<Box<dyn Transform<Error = PipelineError>>>,
}

impl PipelineBuilder {
    /// Create a new, empty builder.
    pub fn new() -> Self {
        todo!("Implement PipelineBuilder::new")
    }

    /// Add a transform to the builder. Consumes and returns `Self` for chaining.
    pub fn add<T>(mut self, transform: T) -> Self
    where
        T: Transform<Error = PipelineError> + 'static,
    {
        todo!("Implement PipelineBuilder::add")
    }

    /// Consume the builder and produce a [`Pipeline`].
    pub fn build(self) -> Pipeline {
        todo!("Implement PipelineBuilder::build")
    }
}

impl Default for PipelineBuilder {
    fn default() -> Self {
        Self::new()
    }
}
