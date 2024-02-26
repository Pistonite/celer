//! Global alert component
import { useRef } from "react";
import { useSelector } from "react-redux";
import {
    Button,
    Dialog,
    DialogActions,
    DialogBody,
    DialogContent,
    DialogSurface,
    DialogTitle,
    DialogTrigger,
    Link,
    Text,
} from "@fluentui/react-components";

import { useKernel } from "core/kernel";
import { viewSelector } from "core/store";
import { sleep } from "low/utils";

export const AppAlert: React.FC = () => {
    const {
        alertTitle,
        alertText,
        alertLearnMoreLink,
        alertOkButton,
        alertCancelButton,
        alertExtraActions,
    } = useSelector(viewSelector);
    const alertMgr = useKernel().getAlertMgr();
    const responseRef = useRef<boolean | string>(false);
    const { RichAlertComponent } = alertMgr;
    if (!alertText && !RichAlertComponent) {
        return null;
    }

    return (
        <Dialog
            open
            modalType="alert"
            onOpenChange={async (_, data) => {
                if (!data.open) {
                    // doing this async just in case
                    await sleep(0);
                    alertMgr.onAction(responseRef.current);
                }
            }}
        >
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>{alertTitle}</DialogTitle>
                    <DialogContent>
                        {RichAlertComponent ? (
                            <RichAlertComponent />
                        ) : (
                            <>
                                <Text block>{alertText}</Text>
                                {alertLearnMoreLink && (
                                    <div style={{ marginTop: 10 }}>
                                        <Link
                                            href={alertLearnMoreLink}
                                            target="_blank"
                                        >
                                            Learn more
                                        </Link>
                                    </div>
                                )}
                            </>
                        )}
                    </DialogContent>
                    <DialogActions fluid={alertExtraActions.length > 0}>
                        {alertOkButton && (
                            <DialogTrigger disableButtonEnhancement>
                                <Button
                                    appearance="primary"
                                    onClick={() => {
                                        responseRef.current = true;
                                    }}
                                >
                                    {alertOkButton}
                                </Button>
                            </DialogTrigger>
                        )}
                        {alertCancelButton && (
                            <DialogTrigger disableButtonEnhancement>
                                <Button
                                    appearance="secondary"
                                    onClick={() => {
                                        responseRef.current = false;
                                    }}
                                >
                                    {alertCancelButton}
                                </Button>
                            </DialogTrigger>
                        )}
                        {alertExtraActions.map((action, i) => (
                            <DialogTrigger key={i} disableButtonEnhancement>
                                <Button
                                    appearance="secondary"
                                    onClick={() => {
                                        responseRef.current = action.id;
                                    }}
                                >
                                    {action.text}
                                </Button>
                            </DialogTrigger>
                        ))}
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    );
};
