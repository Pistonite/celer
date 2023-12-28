//! Resolving `use`s in route

use std::collections::BTreeMap;

use serde_json::{Map, Value};

use crate::env::yield_budget;
use crate::json::{Cast, Coerce, RouteBlob, RouteBlobError};
use crate::macros::async_recursion;
use crate::res::{Loader, ResError, Resource, Use, ValidUse};

use super::Setting;

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
where
    L: Loader,
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
            build_route_from_use(resource, valid_use, use_depth, setting).await
        }
        Some(Use::Invalid(path)) => {
            // is `use` but path is invalid
            ResError::InvalidUse(path).into()
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
                let result = build_route_from_use(resource, valid_use, use_depth, setting).await;
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
                output.push(ResError::InvalidUse(path).into());
            }
            None => {
                // not a use
                let result =
                    build_route_internal(resource, value, use_depth, ref_depth + 1, setting).await;
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
        Err(e) => return e.into(),
    };
    // Load the resource
    let data = match inner_resource.load_structured().await {
        Ok(r) => r,
        Err(e) => return e.into(),
    };

    build_route_internal(
        &inner_resource,
        data,
        use_depth + 1,
        0, // ref depth should be reset inside a new `use`
        setting,
    )
    .await
}
