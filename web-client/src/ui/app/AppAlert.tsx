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

export const AppAlert: React.FC = () => {
    const { alertTitle, alertText, alertLearnMoreLink, alertOkButton, alertCancelButton } =
        useSelector(viewSelector);
    const kernel = useKernel();
    const okRef = useRef<HTMLButtonElement>(null);
    if (!alertText) {
        return null;
    }

    return (
        <Dialog
            open
            modalType="alert"
            onOpenChange={(ev, data) => {
                if (!data.open) {
                    const ok = ev.target === okRef.current;
                    kernel.onAlertAction(ok);
                }
            }}
        >
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>{alertTitle}</DialogTitle>
                    <DialogContent>
                        <Text block>{alertText}</Text>
                        {alertLearnMoreLink && (
                            <Link href={alertLearnMoreLink} target="_blank">
                                Click here to learn more
                            </Link>
                        )}
                    </DialogContent>
                    <DialogActions>
                        {alertCancelButton && (
                            <DialogTrigger disableButtonEnhancement>
                                <Button appearance="secondary">
                                    {alertCancelButton}
                                </Button>
                            </DialogTrigger>
                        )}
                        <DialogTrigger disableButtonEnhancement>
                            <Button ref={okRef} appearance="primary">
                                {alertOkButton}
                            </Button>
                        </DialogTrigger>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    );
};
