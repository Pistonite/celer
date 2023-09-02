//! Packs json blob into [`MapLayerAttr`]

use celerctypes::MapLayerAttr;
use serde_json::Value;

use crate::json::Cast;
use crate::comp::prop;

use super::{PackerResult, PackerError};

macro_rules! check_layer_required_property {
    ($property:expr, $config_index:ident, $layer_index:ident, $obj:ident) => {
        match $obj.remove($property) {
            Some(v) => Ok(v),
            None => Err(PackerError::MissingConfigProperty(
                $config_index,
                format!("{}.{}[{}].{}", prop::MAP, prop::LAYERS, $layer_index, $property),
            )),
        }
    };
}


pub async fn pack_map_layer(value: Value, config_index: usize, layer_index: usize) -> PackerResult<MapLayerAttr> {
    let mut obj = value.try_into_object().map_err(|_| {
        PackerError::InvalidConfigProperty(
            config_index,
            format!("map.layers[{layer_index}]")
        )
    })?;

    let name = check_layer_required_property!(prop::NAME, config_index, layer_index, obj)?;
    let template_url = check_layer_required_property!(prop::TEMPLATE_URL, config_index, layer_index, obj)?;
    let size = check_layer_required_property!(prop::SIZE, config_index, layer_index, obj)?;
    let zoom_bounds = check_layer_required_property!(prop::ZOOM_BOUNDS, config_index, layer_index, obj)?;
    let max_native_zoom = check_layer_required_property!(prop::MAX_NATIVE_ZOOM, config_index, layer_index, obj)?;
    let transform = check_layer_required_property!(prop::TRANSFORM, config_index, layer_index, obj)?;
    let start_z = check_layer_required_property!(prop::START_Z, config_index, layer_index, obj)?;
    let attribution = check_layer_required_property!(prop::ATTRIBUTION, config_index, layer_index, obj)?;

    Err(PackerError::InvalidConfigProperty(
        config_index,
        format!("{}.{}[{layer_index}]", prop::MAP, prop::LAYERS)
    ))

}

#[cfg(test)]
mod test {
    use super::*;

    use serde_json::json;

    #[tokio::test]
    async fn test_invalid_value() {
        let values = vec![
            json!(null),
            json!(false),
            json!(true),
            json!(1),
            json!([]),
            json!(""),
            json!("hello"),
        ];

        for (i, v) in values.into_iter().enumerate() {
            let result = pack_map_layer
                (v, i, i).await;
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    i,
                    format!("map.layers[{i}]")
                ))
            );
        }
    }

    #[tokio::test]
    async fn test_missing_properties() {
        let props = vec![
            prop::NAME,
            prop::TEMPLATE_URL,
            prop::SIZE,
            prop::ZOOM_BOUNDS,
            prop::MAX_NATIVE_ZOOM,
            prop::TRANSFORM,
            prop::START_Z,
            prop::ATTRIBUTION,
        ];

        let mut value = json!({});
        for p in props {
            let result = pack_map_layer(value.clone(), 0, 0).await;
            assert_eq!(result, Err(PackerError::MissingConfigProperty(0, format!("map.layers[0].{p}"))));

            value[p] = json!("test");
        }

    }
}

