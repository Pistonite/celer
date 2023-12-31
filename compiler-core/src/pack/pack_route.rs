//! Packs the route.
//!
//! Resolves `use`s in the route json blob, and leaves the rest untouched.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::json::Cast;
use crate::macros::{async_recursion, maybe_send};
use crate::util::yield_budget;

use super::{PackerError, PackerValue, Resource, Use, ValidUse};

/// Resolve `use`s inside the route json blob
///
/// The following rule is used when seeing a `use`:
/// - If the `use` is inside an array, and the `use` resolves to an inner array, the inner array is injected
/// into the outer array
/// - Otherwise, the resolved value replaces the `use`
///
/// If a `use` cannot be resolved, a [`PackerValue::Err`] is placed in the place of the `use`
pub async fn pack_route(
    project_resource: &Resource,
    route: Value,
    max_use_depth: usize,
    max_ref_depth: usize,
) -> PackerValue {
    pack_route_internal(project_resource, route, 0, 0, max_use_depth, max_ref_depth).await
}

/// Pack a portion of the route
#[maybe_send(async_recursion)]
async fn pack_route_internal(
    // The resource that contains the route
    resource: &Resource,
    // The route blob
    route: Value,
    use_depth: usize,
    ref_depth: usize,
    max_use_depth: usize,
    max_ref_depth: usize,
) -> PackerValue {
    if use_depth > max_use_depth {
        return PackerValue::Err(PackerError::MaxUseDepthExceeded(max_use_depth));
    }
    if ref_depth > max_ref_depth {
        return PackerValue::Err(PackerError::MaxRefDepthExceeded(max_ref_depth));
    }
    let route = match route.try_into_array() {
        Ok(arr) => {
            let mut output = vec![];
            for x in arr.into_iter() {
                yield_budget(64).await;
                match Use::try_from(x) {
                    Ok(Use::Invalid(path)) => {
                        output.push(PackerValue::Err(PackerError::InvalidUse(path)));
                    }
                    Err(x) => {
                        let result = pack_route_internal(
                            resource,
                            x,
                            use_depth,
                            ref_depth + 1,
                            max_use_depth,
                            max_ref_depth,
                        )
                        .await;
                        output.push(result);
                    }
                    Ok(Use::Valid(valid_use)) => {
                        let result = resolve_use(
                            resource,
                            valid_use,
                            use_depth,
                            max_use_depth,
                            max_ref_depth,
                        )
                        .await;
                        match result {
                            PackerValue::Array(arr) => {
                                output.extend(arr);
                            }
                            other => {
                                output.push(other);
                            }
                        }
                    }
                }
            }

            return PackerValue::Array(output);
        }
        Err(route) => route,
    };

    match Use::try_from(route) {
        Ok(Use::Invalid(path)) => PackerValue::Err(PackerError::InvalidUse(path)),
        Err(x) => {
            // array case is covered above
            match x.try_into_object() {
                Ok(obj) => {
                    let mut new_obj = BTreeMap::new();
                    for (key, value) in obj.into_iter() {
                        yield_budget(64).await;
                        let result = pack_route_internal(
                            resource,
                            value,
                            use_depth,
                            ref_depth + 1,
                            max_use_depth,
                            max_ref_depth,
                        )
                        .await;
                        new_obj.insert(key, result);
                    }
                    PackerValue::Object(new_obj)
                }
                Err(x) => {
                    // primitive case
                    PackerValue::Ok(x)
                }
            }
        }
        Ok(Use::Valid(valid_use)) => {
            resolve_use(resource, valid_use, use_depth, max_use_depth, max_ref_depth).await
        }
    }
}

/// Resolve a `use` in the route
async fn resolve_use(
    // The resource that contains the `use`
    resource: &Resource,
    use_prop: ValidUse,
    use_depth: usize,
    max_use_depth: usize,
    max_ref_depth: usize,
) -> PackerValue {
    // Resolve the resource
    let inner_resource = match resource.resolve(&use_prop).await {
        Ok(r) => r,
        Err(e) => return PackerValue::Err(e),
    };
    // Load the resource
    let data = match inner_resource.load_structured().await {
        Ok(r) => r,
        Err(e) => return PackerValue::Err(e),
    };

    pack_route_internal(
        &inner_resource,
        data,
        use_depth + 1,
        0, // ref depth can be reset
        max_use_depth,
        max_ref_depth,
    )
    .await
}
