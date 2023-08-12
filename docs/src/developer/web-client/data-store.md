# Redux Store
The web client uses [Redux](https://redux.js.org/) to manage application state. This application state are used both by React components and outside of React, such as in the map and editor.

The store is created with custom utils that wraps redux. All the store code is at <SourceLink link="web-client/src/data/store"/>

## Declaring a Reducer
Redux reducers are pure functions that computes the next state based on the current state and an action payload.

In celer, reducers can be declared with a payload or without, using the corresponding type definition
```typescript