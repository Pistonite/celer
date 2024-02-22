/// Implementation for ResultHandle and Result
class ResultImpl {
    constructor() {
        this.ok = false;
        this.inner = undefined;
    }

    erase() { return this; }
    put(r) {
        if (this !== r) {
            console.warn("pure/result: Violation! You must pass the same handle to put() as the handle it's invoked from (i.e. x.put(x))!");
            this.ok = r.ok;
            this.inner = r.inner;
        }
    }

    tryCatch(r, fn) {
        if (this !== r) {
            console.warn("pure/result: Violation! You must pass the same handle to tryCatch() as the handle it's invoked from (i.e. x.tryCatch(x))!");
        }
        try {
            r.putOk(fn(r));
        } catch (e) {
            r.putErr(e);
        }
        return r;
    }

    async tryCatchAsync(r, promise) {
        if (this !== r) {
            console.warn("pure/result: Violation! You must pass the same handle to tryCatchAsync() as the handle it's invoked from (i.e. x.tryCatch(x))!");
        }
        try {
            r.putOk(await promise);
        } catch (e) {
            r.putErr(e);
        }
        return r;
    }


    putOk(value) {
        this.ok = true;
        this.inner = value;
        return this;
    }
    voidOk() {
        this.ok = true;
        this.inner = undefined;
        return this;
    }
    putErr(error) {
        this.ok = false;
        this.inner = error;
        return this;
    }
    fork() {
        return new ResultImpl();
    }

    isOk() { return this.ok; }
    isErr() { return !this.ok; }
    ret() { return this; }

    // private

    toStable() {
        if (this.ok) {
            return new StableOk(this.inner);
        }
        return new StableErr(this.inner);
    }
}

/// Implementation for StableOk<T>
class StableOk {
    constructor(inner) {
        this.inner = inner;
    }

    get value() { return this.inner; }
    isOk() { return true; }
    isErr() { return false; }
}

/// Implementation for StableOk<T>
class StableErr {
    constructor(inner) {
        this.inner = inner;
    }

    get error() { return this.inner; }
    isOk() { return false; }
    isErr() { return true; }
}

/// Wrappers

export function tryInvoke(fn) {
    return fn(new ResultImpl()).toStable();
}

export async function tryInvokeAsync(fn) {
    (await fn(new ResultImpl())).toStable();
}

export function tryCatch(fn) {
    try {
        return new StableOk(fn());
    } catch (e) {
        return new StableErr(e);
    }
}

export async function tryCatchAsync(promise) {
    try {
        return new StableOk(await promise);
    } catch (e) {
        return new StableErr(e);
    }
}
