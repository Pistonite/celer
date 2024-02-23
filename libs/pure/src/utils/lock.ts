import Deque from "denque";

/// Ensure you have exclusive access in concurrent code
///
/// Only guaranteed if no one else has reference to the inner object
export class RwLock<T> {
    private inner: T;

    private readers: number = 0;
    private isWriting: boolean = false;
    private readWaiters: Deque<() => void> = new Deque();
    private writeWaiters: Deque<() => void> = new Deque();

    constructor(t: T) {
        this.inner = t;
    }

    /// Acquire a read lock and call fn with the value. Release the lock when fn returns or throws.
    public async scopedRead<R>(fn: (t: T) => Promise<R>): Promise<R> {
        if (this.isWriting) {
            await new Promise<void>((resolve) => {
                // need to check again to make sure it's not already done
                if (this.isWriting) {
                    this.readWaiters.push(resolve);
                    return;
                }
                resolve();
            });
        }
        // acquired
        this.readers++;
        try {
            return await fn(this.inner);
        } finally {
            this.readers--;
            if (this.writeWaiters.length > 0) {
                if (this.readers === 0) {
                    // notify one writer
                    this.writeWaiters.shift()!();
                }
                // don't notify anyone if there are still readers
            } else {
                // notify all readers
                while (this.readWaiters.length > 0) {
                    this.readWaiters.shift()!();
                }
            }
        }
    }

    /// Acquire a write lock and call fn with the value. Release the lock when fn returns or throws.
    ///
    /// fn takes a setter function, which you can use to update the value like `x = set(newX)`
    public async scopedWrite<R>(fn: (t: T, setter: RwLockSetter<T>) => Promise<R>): Promise<R> {
        if (this.isWriting || this.readers > 0) {
            await new Promise<void>((resolve) => {
                // need to check again to make sure it's not already done
                if (this.isWriting || this.readers > 0) {
                    this.writeWaiters.push(resolve);
                    return;
                }
                resolve();
            });
        }
        // acquired
        this.isWriting = true;
        try {
            return await fn(this.inner, (t: T) => {
                this.inner = t;
                return t;
            });
        } finally {
            this.isWriting = false;
            if (this.readWaiters.length > 0) {
                // notify one reader
                this.readWaiters.shift()!();
            } else if (this.writeWaiters.length > 0) {
                // notify one writer
                this.writeWaiters.shift()!();
            }
        }
    }
}

export type RwLockSetter<T> = (t: T) => T;
