//! APIs for the app to use store actions

import { ActionCreatorWithPayload, ActionCreatorWithoutPayload, AnyAction, bindActionCreators as reduxBindActionCreators, Dispatch} from "@reduxjs/toolkit";
import { useDispatch } from "react-redux";

/// TypeScript magic to (correctly) type the result of redux's `bindActionCreators`
export type BindedActionCreators<ActionCreators> = {
    [t in keyof ActionCreators]:
        ActionCreators[t] extends ActionCreatorWithPayload<infer P, infer _>
        ? (payload: P) => void
        : ActionCreators[t] extends ActionCreatorWithoutPayload<infer _>
        ? () => void
        : never;
}

/// TypeScript magic to (correctly) type the result of redux's `bindActionCreators`
type UnbindedActionCreators<ActionCreators> = {
    [t in keyof ActionCreators]:
        ActionCreators[t] extends ActionCreatorWithPayload<infer P, infer N>
        ? ActionCreatorWithPayload<P, N>
        : ActionCreators[t] extends ActionCreatorWithoutPayload<infer N>
        ? ActionCreatorWithoutPayload<N>
        : never;
}


/// Bind actions to dispatch as a React hook
///
/// This is a simple wrapper around redux's `bindActionCreators`
export const useActions = <ActionCreators extends UnbindedActionCreators<ActionCreators>>(actions: ActionCreators): BindedActionCreators<ActionCreators> => {
    const dispatch = useDispatch();
    return bindActions(actions, dispatch);
}

/// Bind actions to dispatch. Can be used outside of React
///
/// This is a direct wrapper around redux's `bindActionCreators` for better typing.
export const bindActions = <ActionCreators>(
    actions: UnbindedActionCreators<ActionCreators>,
    dispatch: Dispatch<AnyAction>
): BindedActionCreators<ActionCreators> => {
    return reduxBindActionCreators(actions, dispatch) as BindedActionCreators<ActionCreators>;
}