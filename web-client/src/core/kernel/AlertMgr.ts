//! Manager for modal alerts

import type { Result } from "@pistonite/pure/result";
import { tryAsync } from "@pistonite/pure/result";

import type { AppDispatcher } from "core/store";
import { viewActions } from "core/store";
import type {
    AlertExtraAction,
    AlertIds,
    AlertMgr,
    AlertOptions,
    BlockingAlertOptions,
    ModifyAlertActionPayload,
    RichAlertOptions,
} from "low/utils";
import { console } from "low/utils";

type AlertCallback = (ok: boolean | string) => void;

/// Timeout needed to clear the previous alert
const ALERT_TIMEOUT = 100;

export class AlertMgrImpl implements AlertMgr {
    private store: AppDispatcher;
    private previousFocusedElement: Element | undefined = undefined;
    private alertCallback: AlertCallback | undefined = undefined;

    public RichAlertComponent: React.ComponentType | undefined = undefined;

    constructor(store: AppDispatcher) {
        this.store = store;
    }

    public onAction(response: boolean | string) {
        this.alertCallback?.(response);
    }

    public show<TExtra extends AlertExtraAction[] = []>({
        title,
        message,
        okButton,
        cancelButton,
        learnMoreLink,
        extraActions,
    }: AlertOptions<TExtra>): Promise<boolean | AlertIds<TExtra>> {
        return new Promise((resolve) => {
            this.initAlert(resolve, undefined);
            this.store.dispatch(
                viewActions.setAlert({
                    title,
                    text: message,
                    okButton: okButton ?? "Ok",
                    cancelButton: cancelButton ?? "Cancel",
                    learnMore: learnMoreLink ?? "",
                    extraActions: extraActions ?? [],
                }),
            );
        });
    }

    public showRich<TExtra extends AlertExtraAction[] = []>({
        title,
        component,
        okButton,
        cancelButton,
        extraActions,
    }: RichAlertOptions<TExtra>): Promise<boolean | AlertIds<TExtra>> {
        return new Promise((resolve) => {
            this.initAlert(resolve, component);
            this.store.dispatch(
                viewActions.setAlert({
                    title,
                    text: "",
                    okButton: okButton ?? "Ok",
                    cancelButton: cancelButton ?? "",
                    learnMore: "",
                    extraActions: extraActions ?? [],
                }),
            );
        });
    }

    public showBlocking<T>(
        { title, component, cancelButton }: BlockingAlertOptions,
        f: () => Promise<T>,
    ): Promise<Result<T, boolean | unknown>> {
        return new Promise((resolve) => {
            let cancelled = false;
            this.initAlert(() => {
                // when alert is notified through user action,
                // it means cancel
                cancelled = true;
                console.info("user cancelled the operation");
                resolve({ err: false });
            }, component);
            this.store.dispatch(
                viewActions.setAlert({
                    title,
                    text: "",
                    okButton: "",
                    cancelButton: cancelButton ?? "Cancel",
                    learnMore: "",
                    extraActions: [],
                }),
            );
            // let the UI update first
            setTimeout(async () => {
                const result = await tryAsync(f);
                if (!cancelled) {
                    this.clearAlertAndThen(() => resolve(result));
                }
            }, ALERT_TIMEOUT);
        });
    }

    public modifyActions(payload: ModifyAlertActionPayload) {
        if (this.alertCallback) {
            this.store.dispatch(viewActions.setAlertActions(payload));
        }
    }

    private initAlert(
        resolve: AlertCallback,
        component: React.ComponentType | undefined,
    ) {
        this.previousFocusedElement = document.activeElement ?? undefined;
        this.alertCallback = (response) => {
            this.clearAlertAndThen(() => resolve(response));
        };
        this.RichAlertComponent = component;
    }

    private clearAlertAndThen(cb: () => void) {
        this.store.dispatch(viewActions.clearAlert());
        this.alertCallback = undefined;
        this.RichAlertComponent = undefined;
        setTimeout(() => {
            const element = this.previousFocusedElement;
            if (
                element &&
                "focus" in element &&
                typeof element.focus === "function"
            ) {
                element.focus();
            }
            cb();
        }, ALERT_TIMEOUT);
    }
}
