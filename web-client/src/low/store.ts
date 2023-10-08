//! low/store
//!
//! Low level store utilities

import {
    Reducer,
    createSlice,
    SliceCaseReducers,
    CreateSliceOptions,
    Draft,
    ActionCreatorWithPayload,
    ActionCreatorWithoutPayload,
    AnyAction,
    bindActionCreators as reduxBindActionCreators,
    Dispatch,
} from "@reduxjs/toolkit";
import { useDispatch } from "react-redux";

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

/// Convenience function for defining a reducer with payload
export const withPayload = <S, P>(
    effect: (state: Draft<S>, payload: P) => void,
): ReducerDeclWithPayload<S, P> => {
    return (state: Draft<S>, action) => {
        return effect(state, action.payload);
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

/// TypeScript magic to (correctly) type the result of redux's `bindActionCreators`
export type BindedActionCreators<ActionCreators> = {
    [t in keyof ActionCreators]: ActionCreators[t] extends ActionCreatorWithoutPayload<
        infer _
    >
        ? () => void
        : ActionCreators[t] extends ActionCreatorWithPayload<infer P, infer _>
        ? (payload: P) => void
        : never;
};

type UnbindedActionCreators<ActionCreators> = {
    [t in keyof ActionCreators]: ActionCreators[t] extends ActionCreatorWithPayload<
        infer P,
        infer N
    >
        ? ActionCreatorWithPayload<P, N>
        : ActionCreators[t] extends ActionCreatorWithoutPayload<infer N>
        ? ActionCreatorWithoutPayload<N>
        : never;
};

/// Bind actions to dispatch as a React hook
///
/// This is a simple wrapper around redux's `bindActionCreators`
export const useActions = <
    ActionCreators extends UnbindedActionCreators<ActionCreators>,
>(
    actions: ActionCreators,
): BindedActionCreators<ActionCreators> => {
    const dispatch = useDispatch();
    return bindActions(actions, dispatch);
};

/// Bind actions to dispatch. Can be used outside of React
///
/// This is a direct wrapper around redux's `bindActionCreators` for better typing.
export const bindActions = <ActionCreators>(
    actions: UnbindedActionCreators<ActionCreators>,
    dispatch: Dispatch<AnyAction>,
): BindedActionCreators<ActionCreators> => {
    return reduxBindActionCreators(
        actions,
        dispatch,
    ) as BindedActionCreators<ActionCreators>;
};
