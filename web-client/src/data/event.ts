//! Event utilities

/// Simple event listener list for notifying
export class EventListenerList<T> {
    private listeners: Array<(payload: T) => void> = [];
    
    public addListener(listener: (payload: T) => void) {
        this.listeners.push(listener);
    }
    
    public removeListener(listener: (payload: T) => void) {
        this.listeners = this.listeners.filter((l) => l !== listener);
    }
    
    public dispatch(payload: T) {
        this.listeners.forEach((l) => l(payload));
    }
}