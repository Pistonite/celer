class ResultImpl {

    constructor(ok, value) {
        this.ok = ok;
        this.value = value;
    }

    isOk() {
        return this.ok;
    }

    isErr() {
        return !this.ok;
    }

    makeOk(value) {
        this.ok = true;
        this.value = value;
        return this;
    }

    makeErr(value) {
        this.ok = false;
        this.value = value;
        return this;
    }

    inner() {
        return this.value ;
    }

    map(fn) {
        if (this.ok) {
            this.value = fn(this.value);
        }
        return this;
    }

    mapErr(fn) {
        if (!this.ok) {
            this.value = fn(this.value);
        }
        return this;
    }
}

export function allocOk(value) {
    return new ResultImpl(true, value);
}

export function allocErr(value) {
    return new ResultImpl(false, value);
}

export function wrap(fn) {
    try {
        return allocOk(fn());
    } catch (e) {
        return allocErr(e);
    }
}

export async function wrapAsync(fn) {
    try {
        return allocOk(await fn());
    } catch (e) {
        return allocErr(e);
    }
}
