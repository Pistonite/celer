//! Packs the route.
//!
//! Resolves `use`s in the route json blob, and leaves the rest untouched.

use serde_json::Value;

use crate::json::Cast;
use crate::util::async_for;

use super::{PackerError, PackerResult, PackerValue, ResourceLoader, ResourceResolver, Use};

/// Resolve `use`s inside the route json blob
///
/// The following rule is used when seeing a `use`:
/// - If the `use` is inside an array, and the `use` resolves to an inner array, the inner array is injected
/// into the outer array
/// - Otherwise, the resolved value replaces the `use`
///
/// If a `use` cannot be resolved, a [`PackerValue::Err`] is placed in the place of the `use`
pub async fn pack_route(
    route: Value,
    resolver: &dyn ResourceResolver,
    loader: &dyn ResourceLoader,
    max_use_depth: usize,
    max_ref_depth: usize,
) -> PackerValue {
    pack_route_internal(route, resolver, loader, 0, 0, max_use_depth, max_ref_depth).await
}

#[async_recursion::async_recursion]
async fn pack_route_internal(
    route: Value,
    resolver: &dyn ResourceResolver,
    loader: &dyn ResourceLoader,
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
            async_for!(x in arr.into_iter(), {
                match Use::from(x) {
                    Use::Invalid(path) => {
                        output.push(PackerValue::Err(PackerError::InvalidUse(path)));
                    }
                    Use::NotUse(x) => {
                        let result = pack_route_internal(
                            x,
                            resolver,
                            loader,
                            use_depth,
                            ref_depth+1,
                            max_use_depth,
                            max_ref_depth
                        ).await;
                        output.push(result);
                    }
                    other => {
                        let result = resolve_use(
                            resolver,
                            loader,
                            &other,
                            use_depth,
                            max_use_depth,
                            max_ref_depth,
                        ).await;
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
            });

            return PackerValue::Array(output);
        }
        Err(route) => route,
    };

    match Use::from(route) {
        Use::Invalid(path) => PackerValue::Err(PackerError::InvalidUse(path)),
        Use::NotUse(x) => PackerValue::Ok(x),
        other => {
            resolve_use(
                resolver,
                loader,
                &other,
                use_depth,
                max_use_depth,
                max_ref_depth,
            )
            .await
        }
    }
}

async fn resolve_use(
    resolver: &dyn ResourceResolver,
    loader: &dyn ResourceLoader,
    use_prop: &Use,
    use_depth: usize,
    max_use_depth: usize,
    max_ref_depth: usize,
) -> PackerValue {
    // Resolve the resource
    let resource = match resolver.resolve(use_prop) {
        Ok(r) => r,
        Err(e) => return PackerValue::Err(e),
    };
    // Load the resource
    let resource_json = match resource.load_structured(loader).await {
        Ok(r) => r,
        Err(e) => return PackerValue::Err(e),
    };
    // Get the resolver to resolve `use`s inside the resource
    let inner_resolver = match resolver.get_resolver(use_prop) {
        Ok(r) => r,
        Err(e) => return PackerValue::Err(e),
    };

    pack_route_internal(
        resource_json,
        inner_resolver.as_ref(),
        loader,
        use_depth + 1,
        0, // ref depth can be reset
        max_use_depth,
        max_ref_depth,
    )
    .await
}
