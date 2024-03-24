//! A SerialEvent tracks an event that should be cancelled
//! if a new event is triggered before the current event completes

import { Result, Void } from "pure/result";

export class SerialEvent {
    private serial: number;
    private onCancel: SerialEventCancelCallback;

    constructor(onCancel?: SerialEventCancelCallback) {
        this.serial = 0;
        if (onCancel) {
            this.onCancel = onCancel;
        } else {
            this.onCancel = () => {};
        }
    }

    public async run<T>(callback: SerialEventCallback<T>): Promise<Result<T, SerialEventCancelToken>> {
        const currentSerial = ++this.serial;
        const shouldCancel = () => {
            if (currentSerial !== this.serial) {
                this.onCancel(currentSerial, this.serial);
                return { err: "cancel" as const };
            }
            return {};
        };
        return await callback(currentSerial, shouldCancel);
    }
}

export type SerialEventCancelCallback = (current: number, latest: number) => void;
export type SerialEventCallback<T=undefined> = (current: number, shouldCancel: () => Void<SerialEventCancelToken>) => Promise<Result<T, SerialEventCancelToken>>;
export type SerialEventCancelToken = "cancel";
