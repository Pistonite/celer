//! Memory Pool

import Denque from "denque";

/// Basic implementation of a memory pool.
/// It does not clean the memory when an object is released.
export class Pool<T> {
    /// The pool of available objects
    private availables: Denque<T> = new Denque<T>();

    /// Returns an object from the pool.
    /// If the pool is empty, returns undefined
    public alloc(): T | undefined {
        return this.availables.shift();
    }

    /// Returns an object to the pool.
    public free(obj: T) {
        this.availables.push(obj);
    }
}
