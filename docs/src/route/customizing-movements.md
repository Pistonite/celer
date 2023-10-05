# Customizing Movements
This section is an extension of [Customizing Lines](./customizing-lines.md),
focusing on the `coord` and `movements` properties for customizing map movements.

## Single Coordinate
Use the `coord` property if the line has a single movement segment.
The coordinate specified should be a [Route Coord](./config/map#coordinate-concepts)

```yaml
- Move somewhere:
    coord: [0, 0]
- Move somewhere else:
    coord: [1, 2, 3]
```
:::tip
This is a shorthand that expands to the `movements` property with a single point.
If both `coord` and `movements` are specified, `coord` will replace `movements`
:::

## Multiple Movements
The `movements` property can be used to specify more than one point of movement:
```yaml
- Move through 3 points:
    movements:
    - [0, 0]
    - [1, 2]
    - [100, 200]
```

## Additional Properties
Each point in the `movements` array can be a mapping with additional properties.
If using this form, the coordinate should be specified with the `to` property.
All properties except for `to` are optional.
|Property|Type|Description|
|-|-|-|
|`to`|[Route Coord](./config/map#coordinate-concepts)|The point to move to|
|`warp`|`boolean`|If `true`, no line is drawn from previous point to this point|
|`exclude`|`boolean`|If `true`, this point is not considered part of the line when fitting the map to document. (Has no effect on the map line itself)|
|`color`|`string`|Similar to the `color` property for the line, change the line color from this point on until next line|
|`icon`|`string`|ID of an icon to show at the movement point. The icon will inherit the priority of the line.|

Examples:
```yaml
movements:
  - to: [10, 20]

  - to: [20, 30]
    warp: true
    exclude: true
    color: blue

  - to: [5, 15]
    exclude: true

  - to: [15, 25]
    color: green

  - to: [30, 40]
    warp: true
    icon: chest
```

## Movement Stack
The movement stack is a new system that replaces the `temp` property of the
older Celer format. This system allows you to save the state of the current movements
and return to it at a later point, even from a different line.

There are 2 operations:
1. `push` - Save the state
2. `pop` - Return to the last saved state

The saved state includes both the position and the color of the line.

Examples:
```yaml
- Line A:
    color: red
    movements:
    - [10, 20]    # at 10,20 now
    - push        # 10,20 + red is saved
    - to: [30,40] # Draw blue line from 10,20 to 30,40
      color: blue
    - pop         # Return to 10,20 and change back to red
                  # (will not draw line when popping)
- Line B:
    movements:
    - pop         # Popping the current movement, and return to nothing
    - [50, 50]    # starting a new segment from 50,50
                  # (will have no line from 10,20 to 50,50)
                  # (color is red here, inherited from previous line)
    - [60, 60]    # draw line from 50,50 to 60,60
    - push
    - [70, 70]    # draw line from 60,60 to 70,70
    - push
    - [80, 80]    # draw line from 70,70 to 80,80
    - pop         # return to 70,70
    - [90, 90]    # draw line from 70,70 to 90,90
    - pop         # return to 60,60
- Line C:
     # will continue from 60,60
```

Functionality-wise, the movement stack is the same as the `warp: true`. However,
it can let you return to a previous state without explicitly warping to the location,
which may save effort when changing the route.

## Presets
You can embed movements from a preset by using the same syntax as you would
for the `presets` property (See [Using Presets](./using-presets.md)).
In this case, the `movements` from the preset will be injected
into the `movements` of the line.
```yaml
- Line:
    movements:
    - [50, 50]
    - _Some::Preset # movements from the preset will be injected here
    - [60, 60]
```
:::tip
When using presets in movements, properties other than `movements` of the
preset are ignored.
:::
