//! Manager for modal alerts

import { AlertExtraAction, ModifyAlertActionPayload } from "core/stage";
import { AppDispatcher, viewActions } from "core/store";

/// Options for showing a simple alert
export type AlertOptions<TExtra extends AlertExtraAction[]> = {
    /// Title of the alert
    title: string;
    /// Message body of the alert
    message: string;
    /// Text for the ok button. Default is "Ok"
    okButton?: string;
    /// Text for the cancel button. Default is no cancel button.
    cancelButton?: string;
    /// Show a learn more link after the message
    learnMoreLink?: string;
    /// Extra actions besides ok and cancel
    extraActions?: TExtra;
}

/// Options for showing a rich (react) alert
export type RichAlertOptions<TExtra extends AlertExtraAction[]> = {
    /// Title of the alert
    title: string;
    /// Body component of the alert
    component: React.ComponentType;
    /// Text for the ok button. Default is "Ok"
    okButton?: string;
    /// Text for the cancel button. Default is "Cancel"
    cancelButton?: string;
    /// Extra actions besides ok and cancel
    extraActions?: TExtra;
}

type IdsOf<T extends AlertExtraAction[]> = T[number]["id"];
type AlertCallback = (ok: boolean | string) => void;

/// Timeout needed to clear the previous alert
const ALERT_TIMEOUT = 100;

export class AlertMgr {
    private store: AppDispatcher;
    private previousFocusedElement: Element | undefined = undefined;
    private alertCallback: AlertCallback | undefined = undefined;
    private RichAlertComponent: React.ComponentType | undefined = undefined;

    constructor(store: AppDispatcher) {
        this.store = store;
    }

    /// Show an alert dialog
    ///
    /// Returns a promise that resolves to true if the user
    /// clicked ok and false if the user clicked cancel.
    ///
    /// If there are extra options, it may resolve to the id of the extra action
    public show<TExtra extends AlertExtraAction[]=[]>({
        title,
        message,
        okButton,
        cancelButton,
        learnMoreLink,
        extraActions,
    }: AlertOptions<TExtra>): Promise<boolean | IdsOf<TExtra>> {
        return new Promise((resolve) => {
            this.initAlert(resolve, undefined);
            this.store.dispatch(viewActions.setAlert({
                title,
                text: message,
                okButton: okButton ?? "Ok",
                cancelButton: cancelButton ?? "Cancel",
                learnMore: learnMoreLink ?? "",
                extraActions: extraActions ?? [],
            }));
        });
    }

    /// Like `show`, but with a custom react component for the body
    public showRich<TExtra extends AlertExtraAction[]=[]>({
        title,
        component,
        okButton,
        cancelButton,
        extraActions,
    }: RichAlertOptions<TExtra>): Promise<boolean | IdsOf<TExtra>> {
        return new Promise((resolve) => {
            this.initAlert(resolve, component);
            this.store.dispatch(viewActions.setAlert({
                title,
                text: "",
                okButton: okButton ?? "Ok",
                cancelButton: cancelButton ?? "",
                learnMore: "",
                extraActions: extraActions ?? [],
            }));
        });
    }

    /// Modify the current alert's actions
    ///
    /// Only effective if a dialog is showing
    public modifyActions(payload: ModifyAlertActionPayload) {
        if (this.alertCallback) {
            this.store.dispatch(viewActions.setAlertActions(payload));
        }
    }

    private initAlert(resolve: AlertCallback, component: React.ComponentType | undefined) {
        this.previousFocusedElement = document.activeElement ?? undefined;
        this.alertCallback = (response) => {
            this.store.dispatch(viewActions.clearAlert());
            this.alertCallback = undefined;
            this.RichAlertComponent = undefined;
            setTimeout(() => {
                const element = this.previousFocusedElement;
                if (element && "focus" in element && typeof element.focus === "function") {
                    element.focus();
                }
                resolve(response);
            }, ALERT_TIMEOUT);
        };
        this.RichAlertComponent = component;
    }

    /// Called from the alert dialog to notify the user action
    public onAction(response: boolean | string) {
        this.alertCallback?.(response);
    }

    public getRichComponent() {
        return this.RichAlertComponent;
    }

}
