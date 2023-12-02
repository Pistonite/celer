# Adding Icons
You can add icons to the document and the map as decoration and markers.

## Configuring an Icon
First, you need to add the icon in the configuration. Let's add an icon with id `my-icon`:
```yaml
config:
- icons:
    my-icon: https://icons.pistonite.org/icon/location.lightroot.none.69a2d5.c1fefe.69a2d5.c1fefe.69a2d5.c1fefe.png
```
:::tip
The icon can be a URL or a local file in the project. See [Icons](./config/icons.md) for the full reference.
:::

## Using the Icon
Now, use the `icon` property to add the icon to a line. The icon will show up in both the document and the map!
```yaml
route:
- Example Section:
  - Line with icon:
      icon: my-icon
```
![image of example](https://cdn.discordapp.com/attachments/951389021114871819/1180397155500961812/image.png?ex=657d4584&is=656ad084&hm=cf45fa428bb55f0b545de91735c684d36adb9f3ce64443cc8a3ebddda3fe92a2&)

You can check out more options for icon configuration [here](./property-reference#icon), including
changing its size in the map or hiding the icon in the document or the map.
