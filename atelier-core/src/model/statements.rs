/*!
Model structures for statements.

*/

use crate::model::shapes::Trait;
use crate::model::values::NodeValue;
use crate::model::{Named, ObjectKey, ShapeID};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Apply {
    id: ShapeID,
    the_trait: Trait,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Metadata {
    id: ObjectKey,
    value: NodeValue,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Named<ShapeID> for Apply {
    fn id(&self) -> &ShapeID {
        &self.id
    }
}

impl Apply {
    pub fn new(id: ShapeID, the_trait: Trait) -> Self {
        Self { id, the_trait }
    }

    pub fn the_trait(&self) -> &Trait {
        &self.the_trait
    }
}

// ------------------------------------------------------------------------------------------------

impl Named<ObjectKey> for Metadata {
    fn id(&self) -> &ObjectKey {
        &self.id
    }
}

impl Metadata {
    pub fn new(id: ObjectKey, value: NodeValue) -> Self {
        Self { id, value }
    }

    pub fn value(&self) -> &NodeValue {
        &self.value
    }
    pub fn set_value(&mut self, value: NodeValue) {
        self.value = value;
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------