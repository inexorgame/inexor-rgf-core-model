use serde::Deserialize;
use serde::Serialize;

use crate::ComponentTypeId;
use crate::Extension;
use crate::NamespacedTypeGetter;
use crate::PropertyType;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;

/// A component defines a set of properties to be applied to entity
/// types and relation types.
#[derive(Clone, Debug)]
pub struct Component {
    /// The type definition of the component.
    pub ty: ComponentTypeId,

    /// Textual description of the component.
    pub description: String,

    /// The properties which are applied on entity or relation instances.
    pub properties: Vec<PropertyType>,

    /// Component specific extensions
    pub extensions: Vec<Extension>,
}

impl Component {
    pub fn new<T: Into<ComponentTypeId>, S: Into<String>>(ty: T, description: S, properties: Vec<PropertyType>, extensions: Vec<Extension>) -> Component {
        Component {
            ty: ty.into(),
            description: description.into(),
            properties,
            extensions,
        }
    }

    /// Constructs a new component with the given name and properties
    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S, description: S, properties: Vec<PropertyType>, extensions: Vec<Extension>) -> Component {
        Component {
            ty: ComponentTypeId::new_from_type(namespace, type_name),
            description: description.into(),
            properties,
            extensions,
        }
    }

    /// Constructs a new component with the given name and properties
    pub fn new_without_extensions<T: Into<ComponentTypeId>, S: Into<String>>(ty: T, description: S, properties: Vec<PropertyType>) -> Component {
        Component {
            ty: ty.into(),
            description: description.into(),
            properties,
            extensions: Vec::new(),
        }
    }

    /// Constructs an component with the given name but without properties
    pub fn new_without_properties<T: Into<ComponentTypeId>, S: Into<String>>(ty: T, description: S, extensions: Vec<Extension>) -> Component {
        Component {
            ty: ty.into(),
            description: description.into(),
            properties: Vec::new(),
            extensions,
        }
    }

    /// Returns true, if the component contains a property with the given name.
    pub fn has_property<S: Into<String>>(&self, property_name: S) -> bool {
        let property_name = property_name.into();
        self.properties.iter().any(|p| p.name == property_name)
    }

    /// Returns true, if the component contains an extension with the given name.
    pub fn has_extension<S: Into<String>>(&self, extension_name: S) -> bool {
        let extension_name = extension_name.into();
        self.extensions.iter().any(|extension| extension.name == extension_name)
    }
}

impl NamespacedTypeGetter for Component {
    fn namespace(&self) -> String {
        self.ty.namespace()
    }

    fn type_name(&self) -> String {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for Component {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }
}

impl From<Component> for TypeDefinition {
    fn from(component: Component) -> Self {
        TypeDefinition {
            type_id_type: TypeIdType::Component,
            namespace: component.ty.namespace(),
            type_name: component.ty.type_name(),
        }
    }
}

impl From<&Component> for ComponentTypeId {
    fn from(component: &Component) -> Self {
        component.ty.clone()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ComponentDao {
    /// The namespace the component belongs to.
    #[serde(default = "String::new")]
    pub namespace: String,

    /// The name of the component.
    #[serde(alias = "name")]
    pub type_name: String,

    /// Textual description of the component.
    #[serde(default = "String::new")]
    pub description: String,

    /// The properties which are applied on entity or relation instances.
    #[serde(default = "Vec::new")]
    pub properties: Vec<PropertyType>,

    /// Component specific extensions
    #[serde(default = "Vec::new")]
    pub extensions: Vec<Extension>,
}

impl From<&ComponentDao> for Component {
    fn from(dao: &ComponentDao) -> Self {
        Self {
            ty: ComponentTypeId::new_from_type(&dao.namespace, &dao.type_name),
            description: dao.description.clone(),
            properties: dao.properties.clone(),
            extensions: dao.extensions.clone(),
        }
    }
}

impl From<&Component> for ComponentDao {
    fn from(component: &Component) -> Self {
        ComponentDao {
            namespace: component.namespace(),
            type_name: component.type_name(),
            description: component.description.clone(),
            properties: component.properties.clone(),
            extensions: component.extensions.clone(),
        }
    }
}
