//! Resolving `use`s in route

use std::collections::BTreeMap;

use serde_json::{Map, Value};

use crate::res::{Use, ValidUse, Resource, Loader, ResError};
use crate::json::{Coerce, Cast};
use crate::macros::async_recursion;
use crate::env::yield_budget;

use super::Setting;

/// A route JSON blob representing the state after resolving `use`s,
/// which could contain errors.
///
/// This is used to expose errors to the compiler, so it can be displayed
/// using the diagnostics API
#[derive(Debug, Clone, PartialEq)]
pub enum RouteBlob {
    /// Primitive value (non-array)
    Prim(Value),
    /// Error
    Err(RouteBlobError),
    /// Array of route blobs
    Array(Vec<RouteBlob>),
    /// Object of route blobs
    Object(BTreeMap<String, RouteBlob>),
}

impl From<Value> for RouteBlob {
    fn from(x: Value) -> Self {
        match x {
            Value::Array(x) => Self::Array(x.into_iter().map(Self::from).collect()),
            Value::Object(x) => Self::Object(
                x.into_iter()
                    .map(|(k, v)| (k, Self::from(v)))
                    .collect(),
            ),
            _ => Self::Prim(x),
        }
    }
}

impl RouteBlob {
    pub fn res_error(e: ResError) -> Self {
        Self::Err(RouteBlobError::ResError(e.to_string()))
    }

    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    pub fn as_array(&self) -> Option<&Vec<RouteBlob>> {
        match self {
            Self::Array(x) => Some(x),
            _ => None,
        }
    }

    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }

    pub fn as_object(&self) -> Option<&BTreeMap<String, RouteBlob>> {
        match self {
            Self::Object(x) => Some(x),
            _ => None,
        }
    }

    /// Flatten the value. Returns the inner value or the first error
    pub fn into_flattened(self) -> Result<Value, RouteBlobError> {
        match self {
            Self::Prim(x) => Ok(x),
            Self::Err(e) => Err(e),
            Self::Array(arr) => {
                let mut output = Vec::with_capacity(arr.len());
                for x in arr.into_iter() {
                    let x = x.into_flattened()?;
                    output.push(x);
                }
                Ok(Value::Array(output))
            }
            Self::Object(obj) => {
                let mut output = Map::new();
                for (key, value) in obj.into_iter() {
                    let value = value.into_flattened()?;
                    output.insert(key, value);
                }
                Ok(Value::Object(output))
            }
        }
    }

    /// Created a flattened copy of the value. Returns the copy or the first error
    pub fn flattened(&self) -> Result<Value, RouteBlobError> {
        match self {
            Self::Prim(x) => Ok(x.clone()),
            Self::Err(e) => Err(e.clone()),
            Self::Array(arr) => {
                let mut output = Vec::with_capacity(arr.len());
                for x in arr {
                    let x = x.flattened()?;
                    output.push(x);
                }
                Ok(Value::Array(output))
            }
            Self::Object(obj) => {
                let mut output = Map::new();
                for (key, value) in obj {
                    let value = value.flattened()?;
                    output.insert(key.clone(), value.clone());
                }
                Ok(Value::Object(output))
            }
        }
    }


}

impl Cast for RouteBlob {
    type Object = BTreeMap<String, RouteBlob>;

    fn try_into_object(self) -> Result<<Self as Cast>::Object, Self> {
        match self {
            Self::Object(obj) => Ok(obj),
            other => {
                #[cfg(debug_assertions)]
                {
                    if let Self::Prim(x) = &other {
                        debug_assert!(!x.is_array() && !x.is_object());
                    }
                }
                Err(other)
            }
        }
    }

    fn try_into_array(self) -> Result<Vec<Self>, Self> {
        match self {
            Self::Array(obj) => Ok(obj),
            other => {
                #[cfg(debug_assertions)]
                {
                    if let Self::Prim(x) = &other {
                        debug_assert!(!x.is_array() && !x.is_object());
                    }
                }
                Err(other)
            }
        }
    }
}

impl Coerce for RouteBlob {
    fn coerce_to_string(&self) -> String {
        match self {
            Self::Prim(x) => x.coerce_to_string(),
            Self::Err(e) => "[object error]".to_string(),
            Self::Array(_) => "[object array]".to_string(),
            Self::Object(_) => "[object object]".to_string(),
        }
    }

    fn coerce_to_repl(&self) -> String {
        match self {
            Self::Prim(x) if x.is_null() => "null".to_string(),
            _ => self.coerce_to_string(),
        }
    }

    fn coerce_truthy(&self) -> bool {
        match self {
            Self::Prim(x) => x.coerce_truthy(),
            Self::Err(_) => false,
            Self::Array(_) => true,
            Self::Object(_) => true,
        }
    }

    fn try_coerce_to_f64(&self) -> Option<f64> {
        match self {
            Self::Prim(x) => x.try_coerce_to_f64(),
            _ => None,
        }
    }

    fn try_coerce_to_u64(&self) -> Option<u64> {
        match self {
            Self::Prim(x) => x.try_coerce_to_u64(),
            _ => None,
        }
    }

    fn try_coerce_to_u32(&self) -> Option<u32> {
        match self {
            Self::Prim(x) => x.try_coerce_to_u32(),
            _ => None,
        }
    }

    fn try_coerce_to_i64(&self) -> Option<i64> {
        match self {
            Self::Prim(x) => x.try_coerce_to_i64(),
            _ => None,
        }
    }

    fn try_coerce_to_bool(&self) -> Option<bool> {
        match self {
            Self::Prim(x) => x.try_coerce_to_bool(),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone, thiserror::Error)]
pub enum RouteBlobError {
    #[error("Failed to load resource: {0}")]
    ResError(String),

    #[error("Max depth of {0} levels of `use` is reached. Please make sure there are no circular dependencies.")]
    MaxUseDepthExceeded(usize),

    #[error("Max reference depth of {0} levels is reached. There might be a formatting error in your project files.")]
    MaxRefDepthExceeded(usize),
}

/// Resolve `use`s inside the route json blob
///
/// The following rule is used when seeing a `use`:
/// - If the `use` is inside an array, and the `use` resolves to an inner array, the inner array is injected
/// into the outer array
/// - Otherwise, the resolved value replaces the `use`
///
/// If a `use` cannot be resolved, the error is placed inside the RouteBlob to defer it to the
/// compiler
pub async fn build_route<L>(
    project_resource: &Resource<'_, L>,
    route: Value,
    setting: &Setting,
) -> RouteBlob 
where L: Loader
{
    build_route_internal(project_resource, route, 0, 0, setting).await
}

/// Pack a portion of the route
#[async_recursion(auto)]
async fn build_route_internal<L>(
    // The resource that contains the route
    resource: &Resource<'_, L>,
    // The route blob
    route: Value,
    use_depth: usize,
    ref_depth: usize,
    setting: &Setting,
) -> RouteBlob
where
    L: Loader,
{
    if use_depth > setting.max_use_depth {
        return RouteBlob::Err(RouteBlobError::MaxUseDepthExceeded(setting.max_use_depth));
    }
    if ref_depth > setting.max_ref_depth {
        return RouteBlob::Err(RouteBlobError::MaxRefDepthExceeded(setting.max_ref_depth));
    }
    let route = match route.try_into_array() {
        Ok(arr) => {
            return build_route_array_value(resource, arr, use_depth, ref_depth, setting).await;
        }
        Err(route) => route,
    };

    match Use::from_value(&route) {
        Some(Use::Valid(valid_use)) => {
            // `use` not inside an array, just resolve it and return
            build_route_from_use(
                resource, 
                valid_use, 
                use_depth, 
                setting,
            ).await
        }
        Some(Use::Invalid(path)) => {
            // is `use` but path is invalid
            RouteBlob::res_error(ResError::InvalidUse(path))
        }
        None => {
            // array case is covered above, so just object or primitive
            match route.try_into_object() {
                Ok(obj) => {
                    let mut new_obj = BTreeMap::new();
                    for (key, value) in obj.into_iter() {
                        yield_budget(64).await;
                        let result = build_route_internal(
                            resource,
                            value,
                            use_depth,
                            ref_depth + 1,
                            setting,
                        )
                        .await;
                        new_obj.insert(key, result);
                    }
                    RouteBlob::Object(new_obj)
                }
                Err(x) => {
                    // primitive case
                    debug_assert!(!x.is_array() && !x.is_object());
                    RouteBlob::Prim(x)
                }
            }
        }
    }
}

async fn build_route_array_value<L>(
    resource: &Resource<'_, L>,
    route: Vec<Value>,
    use_depth: usize,
    ref_depth: usize,
    setting: &Setting,
) -> RouteBlob
where
    L: Loader,
{
    let mut output = vec![];
    for value in route.into_iter() {
        yield_budget(64).await;
        match Use::from_value(&value) {
            Some(Use::Valid(valid_use)) => {
                // for `use` inside array, we need to flatten the resulting array (if it is one)
                let result = build_route_from_use(
                    resource,
                    valid_use,
                    use_depth,
                    setting
                )
                .await;
                match result {
                    RouteBlob::Array(arr) => {
                        output.extend(arr);
                    }
                    other => {
                        output.push(other);
                    }
                }
            }
            Some(Use::Invalid(path)) => {
                // is `use` but path is invalid
                output.push(RouteBlob::res_error(ResError::InvalidUse(path)));
            }
            None => {
                // not a use
                let result = build_route_internal(
                    resource,
                    value,
                    use_depth,
                    ref_depth + 1,
                    setting,
                )
                .await;
                output.push(result);
            }
        }
    }

    RouteBlob::Array(output)
}

/// Resolve a `use` in the route
async fn build_route_from_use<L>(
    // The resource that contains the `use`
    resource: &Resource<'_, L>,
    use_prop: ValidUse,
    use_depth: usize,
    setting: &Setting,
) -> RouteBlob
where
    L: Loader,
{
    // Resolve the resource
    let inner_resource = match resource.resolve(&use_prop) {
        Ok(r) => r,
        Err(e) => return RouteBlob::res_error(e),
    };
    // Load the resource
    let data = match inner_resource.load_structured().await {
        Ok(r) => r,
        Err(e) => return RouteBlob::res_error(e),
    };

    build_route_internal(
        &inner_resource,
        data,
        use_depth + 1,
        0, // ref depth should be reset inside a new `use`
        setting
    )
    .await
}
