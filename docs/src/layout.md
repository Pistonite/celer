# Layout
Want the document on the left? No problem. Each component in Celer is a widget
that you can customize with the layout system. You can create your own layouts
to fit your needs.

## Components
Celer contains the following components:

1. **Header**: Contains the *Title Bar* and the *Toolbar*. The Title Bar displays
the title of the current route being viewed, and the Toolbar has a list of actions available.
See [here](./toolbar.md) for a detailed documentation on the Toolbar.
2. **Document**: Contains the steps and notes of the route being viewed.
3. **Map**: Contains the map with lines, icons and markers to augment the document.

The layout system allows you to adjust the size and position of these components

## Adding Layouts
Celer comes with a default layout which you cannot edit or delete. To make your own layout, first you need to duplicate it:
1. Click on <FluentIcon name="DataTreemap20Regular" /> `Layout` on the Toolbar.
2. Click on <FluentIcon name="Copy20Regular" /> `Duplicate`.
3. Click on <FluentIcon name="DataTreemap20Regular" /> `Layout` on the Toolbar again. The duplicated layout should now be selected.
:::tip
You can always add more layouts by duplicating existing ones.
:::

## Editing Layouts
To edit a layout:
1. Click on <FluentIcon name="DataTreemap20Regular" /> `Layout` on the Toolbar.
2. Click on <FluentIcon name="Edit20Regular" /> `Edit`.
:::tip
You cannot edit the default layout! Duplicate it first to edit the layout.
:::

You should now be in the layout editing mode. In this mode, each panel
becomes a draggable and resizable widget. You can drag a widget to move it, or 
drag on the lower-right resize handle to resize it.

When finished editing, Click on <FluentIcon name="DataTreemap20Regular" /> `Layout` > <FluentIcon name="Save20Regular"/> `Finish`.

## Deleting Layouts
To delete a layout:
1. Click on <FluentIcon name="DataTreemap20Regular" /> `Layout` on the Toolbar.
2. Select the layout you want to delete.
3. Click on <FluentIcon name="DataTreemap20Regular" /> `Layout` on the Toolbar.
4. Click on <FluentIcon name="Delete20Regular" /> `Delete`.

## Adjusting Toolbar Location
By default, the Toolbar is attached above the Document panel. You can adjust it to be attached to another panel,
and change it to be either above or below that panel.

1. Click on <FluentIcon name="Window20Regular" /> `Change toolbar location` on the Toolbar.
2. Select the panel you want the toolbar to attach to and whether it should be on the top or bottom of the panel.
