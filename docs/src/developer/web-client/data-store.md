# Redux Store
The web client uses [Redux](https://redux.js.org/) to manage application state. This application state are used both by React components and outside of React, such as in the map and editor.

The store is created with custom utils that wraps redux. All the store code is at <SourceLink link="web-client/src/data/store"/>

## Declaring a Reducer
Redux reducers are pure functions that computes the next state based on the current state and an action payload.

In celer, reducers can be declared with a payload or without, using the corresponding type definition
```typescript
/// Simplified typedef for redux reducer declaration
///
/// Use this and `withPayload` to define a reducer with payload(which will be packaged to an action)
/// For example:
/// ```
/// /// Reducer for doSomething action
/// export const doSomething: ReducerDeclWithPayload<MyState, {
///     /// value of the payload
///     value: number
/// }> = withPayload((state, payload) => {
///     state.value = payload.value;
/// });
/// ```
export type ReducerDeclWithPayload<S, P> = (state: Draft<S>, action: Action<P>) => void;
/// Simplified typedef for redux reducer declaration
///
/// Use this to define a reducer without payload. No need to wrap the function with `withPayload`
export type ReducerDecl<S> = (state: Draft<S>) => void;

```

One example is the `addAndSwitchLayout` reducer in the layout setting slice (<SourceLink link="web-client/src/data/store/settings/layout/reducers.ts"/>). Note that this reducer operates on the `LayoutSettings` slice, which is part of the `SettingsStore` slice.

```typescript
/// Add a layout and switch to it
export const addAndSwitchLayout: ReducerDeclWithPayload<LayoutSettings, {
    /// The layout to add
    layout: Layout
}> = withPayload((state, { layout }) => { ... });
```

The reducers declared this way will eventually be passed into redux's `createSlice` function, which packages them into `ActionCreator`s. Redux uses `immer` internally for the immutable update pattern, so you can write the reducer as if it is mutating the state (as indicated by `Draft<S>` in the declaration)

## Creating a Slice
A slice consists of a set of reducers and the state type the reducers operate on. The state can be any shape that makes sense.

To create a slice, use the `configureSlice` helper function, which is a wrapper around redux's `createSlice`. Supply the name, initial state, and the reducers. For example, here's how the `settings` slice is created.

```typescript
/// The setting state slice
export const {
    settingsReducer,
    settingsActions,
    settingsSelector
} = configureSlice({
    name: "settings",
    initialState: loadState(),
    reducers: {
        ...LayoutReducers,
        setCurrentViewingLayoutTest
    }
});

// re-exports
export * from "./layout/defaults";
export * from "./layout/util";
```

`configureSlice` will return 3 things for us:
1. `xxxReducer` - The slice's reducer configuration, which can be used to create the store later. This wraps the reducer created with `createSlice` with the name of the slice, so we don't have to worry about naming the reducer later in the store.
2. `xxxActions` - The slice's action creators, which can be used to dispatch actions later.
3. `xxxSelector` - The selector function used to get the slice state from the store state.


## Creating Store
To create the store, import all the reducers and use redux's `configureStore`. Note that we can use the imported reducers directly, as they are already wrapped with the slice name.

```typescript
/// The store
export const store = configureStore({
    reducer: {
        ...settingsReducer,
        ...toolbarReducer
    }
});
```

The store also has additional setups like subscribing to updates. See <SourceLink link="web-client/src/data/store/configureStore.ts" /> for more details.

## Usage with React
### Access state
To access the store state, use the `useSelector` hook from `react-redux` along with the selector exported by the slice:

```typescript
import { useSelector } from "react-redux";

const Foo = () => {
    const { someSettingValue } = useSelector(settingsSelector);
}
```

### Dispatch actions
To access the actions, there's a custom hook `useActions` that wraps `useDispatch` and `bindActionCreators`.

```typescript
/// Bind actions to dispatch as a React hook
///
/// This is a simple wrapper around redux's `bindActionCreators`
export const useActions = <ActionCreators extends UnbindedActionCreators<ActionCreators>>(actions: ActionCreators): BindedActionCreators<ActionCreators> => {
    const dispatch = useDispatch();
    return bindActions(actions, dispatch);
}
```

Example usage of the `useActions` hook in a component:

```typescript
import { useActions } from "data/store";

const Foo = () => {
    const { addAndSwitchLayout } = useActions(settingsActtions);

    // ... later
    addAndSwitchLayout(payload);
}
```

## Usage outside of React

TODO but the idea is to use store.getState() and store.dispatch() directly.

## Slices

### SettingsStore

### ToolbarStore
