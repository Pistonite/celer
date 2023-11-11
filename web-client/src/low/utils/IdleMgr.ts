/// Callback to execute an idle event
export type IdleFunction = (isLong: boolean, duration: number) => Promise<void>;

/// Idle manager
///
/// The idle manager runs callbacks when the application is idle.
///
/// There are 2 types of idle events: short and long.
///
/// Short events are triggered when the application is idle for a short period of time.
/// If the application keeps being idle, then events triggered will be long events.
/// Long events are less frequent, and heavier tasks can be executed.
///
/// When the manager decides the application is in long idle, it only fires long events and not short events.
export class IdleMgr {
    // === config values ===
    /// Interval threshold for an idle to be considered long
    private longIdleTime: number;
    /// Initial interval
    private initialInterval: number;
    /// Every time an idle event is fired X times, the interval is multiplied by this factor
    private intervalMultiplier: number;
    /// The X times for the interval multiplier
    private eventsCountForNextInterval: number;
    /// Maximum interval
    private maxInterval: number;

    // === state values ===
    private callback: IdleFunction;
    private handle: number | undefined;

    private currentInterval: number;
    private eventsFiredInCurrentInterval: number;
    private idleDuration: number;
    /// Like a semaphore. Will only fire events if this is 0
    private pauseCount: number;

    constructor(
        longIdleTime: number,
        initialInterval: number,
        intervalMultiplier: number,
        eventsCountForNextInterval: number,
        maxInterval: number,
        callback: IdleFunction) {
        this.longIdleTime = longIdleTime;
        this.initialInterval = initialInterval;
        this.intervalMultiplier = intervalMultiplier;
        this.eventsCountForNextInterval = eventsCountForNextInterval;
        this.maxInterval = maxInterval;
        this.callback = callback;
        this.pauseCount = 1;
        this.currentInterval = this.initialInterval;
        this.eventsFiredInCurrentInterval = 0;
        this.idleDuration = 0;
    }

    /// Start the idle manager. Events will only fire after calling this
    public start() {
        this.pauseCount--;
        if (this.pauseCount > 0) {
            return;
        }
        this.restartIdleTimer();
    }

    /// Stop the idle manager. Events will not fire after calling this
    ///
    /// Do not use this for temporary pauses since you need to manually call start() again.
    /// Use pauseIdleScope instead.
    public stop() {
        this.pauseCount++;
        if (this.pauseCount > 0) {
            this.cancelPendingIdle();
        }
    }

    /// Notify the idle manager that an activity has occurred and the application is not idling
    public notifyActivity() {
        this.currentInterval = this.initialInterval;
        this.eventsFiredInCurrentInterval = 0;
        this.idleDuration = 0;
        this.restartIdleTimer();
    }

    public async pauseIdleScope<T>(f: () => Promise<T>): Promise<T> {
        this.stop();
        try {
            return await f();
        } finally {
            this.start();
        }
    }

    private restartIdleTimer() {
        this.cancelPendingIdle();
        this.handle = window.setTimeout(() => {
            this.handle = undefined;
            if (this.pauseCount > 0) {
                return;
            }
            // update interval time.
            if (this.currentInterval < this.maxInterval) {
                this.eventsFiredInCurrentInterval++;
                if (
                    this.eventsFiredInCurrentInterval >= this.eventsCountForNextInterval
                ) {
                    this.currentInterval *= this.intervalMultiplier;
                    this.eventsFiredInCurrentInterval = 0;
                }
            }
            // update duration
            this.idleDuration += this.currentInterval;
            this.callback(
                this.idleDuration >= this.longIdleTime,
                this.idleDuration,
            )
                .catch(console.error)
                .finally(() => {
                    this.restartIdleTimer();
                });
        }, this.currentInterval);
    }

    private cancelPendingIdle() {
        if (this.handle !== undefined) {
            window.clearTimeout(this.handle);
            this.handle = undefined;
        }
    }
}
