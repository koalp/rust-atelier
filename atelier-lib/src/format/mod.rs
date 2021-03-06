/*!
Child modules that implement `ModelReader` and `ModelWriter` for specific representations.
*/

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "json")]
pub use atelier_json as json;

#[cfg(feature = "openapi")]
pub use atelier_openapi as openapi;

#[cfg(feature = "rdf")]
pub use atelier_rdf as rdf;

#[cfg(feature = "smithy")]
pub use atelier_smithy as smithy;

#[cfg(feature = "uml")]
pub mod plant_uml;
