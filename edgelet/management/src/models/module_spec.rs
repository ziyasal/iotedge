/*
 * IoT Edge Management API
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 2018-06-28
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleSpec {
    /// The name of a the module.
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "type")]
    type_: String,
    #[serde(rename = "config")]
    config: ::models::Config,
}

impl ModuleSpec {
    pub fn new(name: String, type_: String, config: ::models::Config) -> Self {
        ModuleSpec {
            name,
            type_,
            config,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_type(&mut self, type_: String) {
        self.type_ = type_;
    }

    pub fn with_type(mut self, type_: String) -> Self {
        self.type_ = type_;
        self
    }

    pub fn type_(&self) -> &String {
        &self.type_
    }

    pub fn set_config(&mut self, config: ::models::Config) {
        self.config = config;
    }

    pub fn with_config(mut self, config: ::models::Config) -> Self {
        self.config = config;
        self
    }

    pub fn config(&self) -> &::models::Config {
        &self.config
    }
}
