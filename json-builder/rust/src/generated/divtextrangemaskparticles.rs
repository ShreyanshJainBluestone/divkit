// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivTextRangeMaskParticles {
    pub color: Expression<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<Expression<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_animated: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_size: Option<DivFixedSize>,
    #[serde(default = "DivTextRangeMaskParticles::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivTextRangeMaskParticlesBuilder {
    pub color: Option<Expression<String>>,
    pub density: Option<Expression<f64>>,
    pub is_animated: Option<Expression<bool>>,
    pub is_enabled: Option<Expression<bool>>,
    pub particle_size: Option<DivFixedSize>,
    pub r#type: Option<String>,
}

impl DivTextRangeMaskParticles {
    pub fn builder() -> DivTextRangeMaskParticlesBuilder {
        DivTextRangeMaskParticlesBuilder::default()
    }
    pub fn default_density() -> Expression<f64> { Expression::value(0.8_f64) }
    pub fn default_is_animated() -> Expression<bool> { Expression::value(true) }
    pub fn default_is_enabled() -> Expression<bool> { Expression::value(true) }
    pub fn default_particle_size() -> DivFixedSize { serde_json::from_str(r#"{"type":"fixed","value":1}"#).unwrap() }
    pub fn default_type() -> String { "particles".to_string() }
}

impl DivTextRangeMaskParticlesBuilder {
    pub fn color(mut self, value: Expression<String>) -> Self {
        self.color = Some(value);
        self
    }
    pub fn density(mut self, value: Expression<f64>) -> Self {
        self.density = Some(value);
        self
    }
    pub fn is_animated(mut self, value: Expression<bool>) -> Self {
        self.is_animated = Some(value);
        self
    }
    pub fn is_enabled(mut self, value: Expression<bool>) -> Self {
        self.is_enabled = Some(value);
        self
    }
    pub fn particle_size(mut self, value: DivFixedSize) -> Self {
        self.particle_size = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivTextRangeMaskParticles {
        DivTextRangeMaskParticles {
            color: self.color.expect("missing required field 'color'"),
            density: self.density.or_else(|| Some(DivTextRangeMaskParticles::default_density())),
            is_animated: self.is_animated.or_else(|| Some(DivTextRangeMaskParticles::default_is_animated())),
            is_enabled: self.is_enabled.or_else(|| Some(DivTextRangeMaskParticles::default_is_enabled())),
            particle_size: self.particle_size.or_else(|| Some(DivTextRangeMaskParticles::default_particle_size())),
            r#type: String::from("particles"),
        }
    }
}
