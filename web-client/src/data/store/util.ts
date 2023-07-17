//! Utilitiy types and functions for setting up redux store
//!
//! Note: import this as "data/store/util" to prevent circular dependency

import {
    Reducer,
    createSlice,
    SliceCaseReducers,
    CreateSliceOptions,
    Draft,
} from "@reduxjs/toolkit";

/// Simplified typedef for redux action
type Action<P> = {
    type: string;
    payload: P;
};

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
export type ReducerDeclWithPayload<S, P> = (
    state: Draft<S>,
    action: Action<P>,
) => void;
/// Simplified typedef for redux reducer declaration
///
/// Use this to define a reducer without payload. No need to wrap the function with `withPayload`
export type ReducerDecl<S> = (state: Draft<S>) => void;

/// Helper type for inner function of `withPayload`
type ReducerEffect<S, P = undefined> = P extends undefined
    ? ReducerDecl<S>
    : (state: Draft<S>, payload: P) => void;

/// Convenience function for defining a reducer with payload
export const withPayload = <S, P>(
    effect: ReducerEffect<S, P>,
): ReducerDeclWithPayload<S, P> => {
    return (state, action) => {
        effect(state, action.payload);
    };
};

type ParentState<Name extends string, State> = {
    [t in Name]: State;
};

/// Return type definition for configureSlice
type SliceConfiguration<Name extends string, State, Actions> = {
    [t in `${Name}Reducer`]: {
        [t in Name]: Reducer<State>;
    };
} & {
    [t in `${Name}Actions`]: Actions;
} & {
    [t in `${Name}Selector`]: (
        state: ParentState<Name, State>,
    ) => Readonly<State>;
};

/// A wrapper for createSlice that defines exports directly
export const configureSlice = <
    Name extends string,
    State,
    Reducers extends SliceCaseReducers<State>,
>(
    args: CreateSliceOptions<State, Reducers, Name>,
) => {
    const slice = createSlice(args);
    return {
        [`${args.name}Reducer`]: {
            [args.name]: slice.reducer,
        },
        [`${args.name}Actions`]: slice.actions,
        [`${args.name}Selector`]: (state: ParentState<Name, State>) =>
            state[args.name],
    } as SliceConfiguration<Name, State, typeof slice.actions>;
};
