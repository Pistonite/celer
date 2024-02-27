import Deque from "denque";

/// Ensure you have exclusive access in concurrent code
///
/// Only guaranteed if no one else has reference to the inner object
///
/// It can take a second type parameter to specify interface with write methods
export class RwLock<TRead, TWrite extends TRead = TRead> {
    /// This is public so inner object can be accessed directly
    /// ONLY SAFE in sync context
    public inner: TWrite;

    private readers: number = 0;
    private isWriting: boolean = false;
    private readWaiters: Deque<() => void> = new Deque();
    private writeWaiters: Deque<() => void> = new Deque();

    constructor(t: TWrite) {
        this.inner = t;
    }

    /// Acquire a read (shared) lock and call fn with the value. Release the lock when fn returns or throws.
    public async scopedRead<R>(fn: (t: TRead) => Promise<R>): Promise<R> {
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

    /// Acquire a write (exclusive) lock and call fn with the value. Release the lock when fn returns or throws.
    ///
    /// fn takes a setter function as second parameter, which you can use to update the value like `x = set(newX)`
    public async scopedWrite<R>(
        fn: (t: TWrite, setter: (t: TWrite) => TWrite) => Promise<R>,
    ): Promise<R> {
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
            return await fn(this.inner, (t: TWrite) => {
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
