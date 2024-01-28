//! Debouncing utils
import { useCallback, useRef } from "react";

/// A general-purpose debouncer
///
/// This is useful for debouncing expensive operations such as recreating the map
/// when changing the settings.
///
/// This is particularly useful outside of React where useTransition hook
/// is not available.
//
/// The dispatch method returns a promise that resolvese to the result
/// of a callback succesfully dispatched. When a pending callback is cancelled,
/// a fallback value can be computed using another callback.
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export class Debouncer<TCallback extends (...args: any[]) => any = () => void> {
    /// The timeout handle
    private handle: number | undefined;
    /// The resolve function of the pending promise
    private resolve:
        | ((value: ReturnType<TCallback> | undefined) => void)
        | undefined;
    /// The delay in ms
    private delay: number;
    /// The callback action
    private callback: TCallback;
    /// The cancel fallback value
    private cancelFallback: TCallback | undefined;

    constructor(
        delay: number,
        callback: TCallback,
        cancelFallback?: TCallback,
    ) {
        this.delay = delay;
        this.callback = callback;
        this.cancelFallback = cancelFallback;
    }

    /// Trigger the callback after delay if not triggered again
    ///
    /// The returned promise resolves when the callback finishes. If the callback
    /// is cancelled, the promise resolves to the result of the cancel fallback, or undefined
    /// if there is no cancel fallback.
    public dispatch(
        ...args: unknown[]
    ): Promise<ReturnType<TCallback> | undefined> {
        return new Promise((resolve) => {
            this.cancelPending();
            if (this.resolve) {
                this.resolve(this.cancelFallback?.());
            }
            this.resolve = resolve;
            this.handle = window.setTimeout(() => {
                this.resolve = undefined;
                resolve(this.callback(...args));
            }, this.delay);
        });
    }

    /// Cancel the pending callback if there is one
    public cancelPending() {
        if (this.handle !== undefined) {
            clearTimeout(this.handle);
        }
    }
}

/// Sometimes in React, it's difficult to get a stable callback.
/// So use this hook instead
export const useDebouncer = (delay: number) => {
    const handle = useRef<number | undefined>(undefined);

    const dispatch = useCallback(
        (fn: () => void) => {
            if (handle.current !== undefined) {
                clearTimeout(handle.current);
            }
            handle.current = window.setTimeout(fn, delay);
        },
        [delay, handle],
    );

    return dispatch;
};
