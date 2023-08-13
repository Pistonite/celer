# Configuration
This section documents what you can add in the `config` section in `project.yaml`.
You don't need to understand all the configuration at once. Feel free to come back
here to explore.

## Map configuration

The `config` property is an array of configuration objects.
You can write the configuration objects in `project.yaml`, or load a file in the project
or from GitHub.

Example:
```yaml
# project.yaml
config:
- use: Pistonite/celer/presets/botw-map.yaml
- use: Pistonite/celer/presets/botw-presets.yaml
- map:
    layers:
```
