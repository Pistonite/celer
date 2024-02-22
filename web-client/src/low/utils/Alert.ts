import { Result, ResultHandle } from "pure/result";

export type AlertExtraAction = {
    id: string;
    text: string;
};
export type ModifyAlertActionPayload = {
    okButton?: string;
    cancelButton?: string;
    extraActions?: AlertExtraAction[];
};

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
};

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
};

/// Options to show a blocking alert while another operation is running
export type BlockingAlertOptions = {
    /// Title of the alert
    title: string;
    /// Body component of the alert
    component: React.ComponentType;
    /// Text for the cancel button. Default is "Cancel"
    cancelButton?: string;
};

export type AlertIds<T extends AlertExtraAction[]> = T[number]["id"];

export interface AlertMgr {
    /// Show an alert dialog
    ///
    /// Returns a promise that resolves to true if the user
    /// clicked ok and false if the user clicked cancel.
    ///
    /// If there are extra options, it may resolve to the id of the extra action
    show<TExtra extends AlertExtraAction[]=[]>(
        options: AlertOptions<TExtra>
    ): Promise<boolean | AlertIds<TExtra>>;

    /// Like `show`, but with a custom react component for the body
    showRich<TExtra extends AlertExtraAction[]=[]>(
        options: RichAlertOptions<TExtra>
    ): Promise<boolean | AlertIds<TExtra>>;

    /// Show a blocking alert and run f
    ///
    /// The promise will resolve to the result of f, or Err(false) if the user
    /// cancels.
    ///
    /// If f throws, the alert will be cleared, and Err(e) will be returned.
    showBlocking<T>(
        r: ResultHandle,
        options: BlockingAlertOptions,
        fn: () => Promise<T>
    ): Promise<Result<T, unknown>>;

    /// Modify the current alert's actions
    ///
    /// Only effective if a dialog is showing
    modifyActions(payload: ModifyAlertActionPayload): void;

    /// Called from the alert dialog to notify the user action
    onAction(response: boolean | string): void;

    /// Get the rich component if a rich alert is showing
    readonly RichAlertComponent?: React.ComponentType;
}
