/// A general-purpose debouncer
///
/// This is useful for debouncing expensive operations such as recreating the map
/// when changing the settings.
///
/// This is particularly useful outside of React where useTransition hook
/// is not available.
export class Debouncer {
    /// The timeout handle
    private handle: number | undefined;
    /// The delay in ms
    private delay: number;
    /// The callback action
    private callback: () => void;

    constructor(delay: number, callback: () => void) {
        this.delay = delay;
        this.callback = callback;
    }

    /// Trigger the callback after delay if not triggered again
    public dispatch() {
        if (this.handle !== undefined) {
            clearTimeout(this.handle);
        }
        this.handle = window.setTimeout(this.callback, this.delay);
    }
}
