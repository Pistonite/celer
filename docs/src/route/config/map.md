# Map Configuration
The `map` property in `config` is used for configuring map properties.
Only one config in the array can specify configuration for map, otherwise
the compiler will error.

An example of a map configuration: <SourceLink link="presets/botw-map.yaml" />

All properties below are required, unless specified otherwise.

## Coordinate Concepts
1. Route coordinates
    - Specified as 2 axes or 3 axes in the route.
    - Layer independent
2. Game coordinates: 
    - `[x, y, z]` that usually correspond to the scale in the game. `x` is horizontal, `y` is vertical, and `z` is height.
    - Transformed from route coordinates using the `coord-map` property
    - Layer independent
3. Raster coordinates: 
    - `[x, y]` on the tileset image. (All positive)
    - Transformed from the `x` and `y` of the game coordinates using the `transform` property of the layer that the game coord is on.
    - Layer dependent
    - For more info, see https://github.com/commenthol/leaflet-rastercoords.
4. Latitude and Longtitude
    - Used internally in leaflet map
    - `leaflet-rastercoords` handles the transformation
:::tip
Routes will usually only work with route coordinates. Map preset makers need to create the correct transform for game and raster coordinates for the routers.
:::

### Coord Map
In the example below from Breath of the Wild, when 3 axes are specified, the middle
is `z` (height). This transform allow users to specify (x, y, z) in the order they
know from the game.
```yaml
config:
  map:
    coord-map:
      2d: ["x", "y"]
      3d: ["x", "z", "y"]
```
:::tip
You can also specify one axis multiple times. The axes not specified will be 0
:::

## Initials
Use these properties to specify the initial condition of the map.
|Property|Type|Description|
|-|-|-|
|`initial-coord`|Game coord|Center of the map when the route is loaded|
|`initial-zoom`|`number`|Zoom of the map when the route is loaded. Smaller is zoomed out|
|`initial-color`|`string`|Default color of the line|

Example:
```yaml
config:
  map:
    initial-coord: [50, 60, 70]
    initial-zoom: 3
    initial-color: blue
```


## Layers
Celer supports multiple layers of the map governed by `z` coordinate (height).

The `layers` property should be an array of layer objects, each with the following properties:
|Property|Type|Description|
|-|-|-|
|`name`|`string`|Name of the layer. Visible in the layer switch UI|
|`template-url`|`string`|Tileset URL of this layer. See https://leafletjs.com/reference.html#tilelayer|
|`size`|`[width, height]`|Raster coord size of this layer. Celer uses `leaflet-rastercoords` to create layers. See https://github.com/commenthol/leaflet-rastercoords for how to get this from the layer image|
|`zoom-bounds`|`[min, max]`|Min and max zoom of this layer|
|`max-native-zoom`|`number`|Max supported native zoom of the tileset. Higher zoom levels are produced by scaling the tile|
|`transform`|mapping|See [below](#layer-transform)|
|`start-z`|`number`|The lowest `z` value where this layer starts. This is ignored for the first layer|
|`attribution`|mapping|A `link` property for link to the tileset provider, and an optional `copyright` boolean property for if the tileset is copyrighted|
:::warning
Celer does not support hosting tilesets. If your tileset is not hosted publicly, you need to host it yourself and use the URL in the config.
:::

### Layer Transform
The `transform` property has 2 subproperties: `scale` and `translate`.

Suppose scale is `[a, b]` and translate is `[c, d]`. The raster coord is transformed from game coord as:
```
(x, y) -> (x * a + c, y * b + d)
```

