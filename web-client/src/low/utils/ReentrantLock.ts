/// A lock to prevent concurrent async operations
///
/// The lock uses a token to prevent deadlocks. It will not block
/// if the caller provides a token that currently holds the lock.
export class ReentrantLock {
    private name: string;
    private lockingToken: number | undefined = undefined;
    private nextToken = 0;
    private waiters: ((x: unknown) => void)[] = [];

    constructor(name: string) {
        this.name = name;
    }

    /// Acquires the lock and call f.
    ///
    /// If the lock is held by another token, this will wait for the lock to be released.
    public async lockedScope<T>(
        token: number | undefined,
        f: (token: number) => Promise<T>,
    ): Promise<T> {
        if (this.lockingToken !== undefined && token !== this.lockingToken) {
            if (token !== undefined) {
                window.console.error(
                    `invalid lock token passed to ${this.name} lock!`,
                );
            }
            // someone else is holding the lock, wait for it to be released
            await new Promise((resolve) => {
                if (this.lockingToken === undefined) {
                    resolve(undefined);
                }
                this.waiters.push(resolve);
            });
        }
        if (this.lockingToken === undefined) {
            if (token !== undefined) {
                window.console.error(
                    `invalid lock token passed to ${this.name} lock!`,
                );
            }
            // no one is holding the lock, acquire it
            this.lockingToken = ++this.nextToken;
            let returnVal;
            try {
                returnVal = await f(this.lockingToken);
            } finally {
                this.lockingToken = undefined;
                const waiters = this.waiters;
                this.waiters = [];
                waiters.forEach((w) => w(undefined));
            }
            return returnVal;
        } else {
            return await f(this.lockingToken);
            // do not release the lock afterwards
        }
    }
}
