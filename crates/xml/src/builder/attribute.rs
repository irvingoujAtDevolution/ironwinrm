use std::{borrow::Cow, collections::HashMap};

/// Represents an XML attribute with a name and value.
#[derive(Debug, Clone)]
pub struct Attribute<'a> {
    /// The name of the attribute.
    name: &'a str,
    /// The value of the attribute.
    value: Cow<'a, str>,

    namespace: Option<crate::builder::Namespace<'a>>,
}

impl<'a> Attribute<'a> {
    /// Creates a new instance of `Attribute`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the attribute.
    /// * `value` - The value of the attribute.
    ///
    /// # Example
    ///
    /// ```
    /// use xml::builder::Attribute;
    /// let attribute = Attribute::new("name", "value");
    /// ```
    pub fn new(name: &'a str, value: impl Into<Cow<'a, str>>) -> Self {
        Attribute {
            name,
            value: value.into(),
            namespace: None,
        }
    }

    pub fn new_with_namespace(
        name: &'a str,
        value: impl Into<Cow<'a, str>>,
        namespace: Option<impl Into<crate::builder::Namespace<'a>>>,
    ) -> Self {
        Attribute {
            name,
            value: value.into(),
            namespace: namespace.map(|ns| ns.into()),
        }
    }

    pub fn set_namespace(mut self, namespace: impl Into<crate::builder::Namespace<'a>>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    pub fn get_namespaces(
        &self,
        namespaces_set: &mut std::collections::HashSet<crate::builder::Namespace<'a>>,
    ) {
        if let Some(namespace) = &self.namespace {
            namespaces_set.insert(namespace.clone());
        }
    }
}

impl crate::builder::NamespaceFmt for Attribute<'_> {
    /// Formats the attribute as a string in the format `name="value"`.
    fn ns_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        alias_map: Option<&HashMap<super::namespace::Namespace<'_>, &str>>,
    ) -> std::fmt::Result {
        let namespace_alias = if let Some(alias_map) = alias_map {
            self.namespace
                .as_ref()
                .and_then(|ns| alias_map.get(ns))
                .copied()
        } else {
            return Err(std::fmt::Error);
        };

        let name = if let Some(alias) = namespace_alias {
            format!("{}:{}", alias, self.name)
        } else {
            self.name.to_string()
        };

        write!(f, " {}=\"{}\"", name, self.value)?;
        Ok(())
    }
}
