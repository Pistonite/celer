//! Global alert component
import "./AppAlert.css";
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
                            <div className="alert-link">
                                <Link href={alertLearnMoreLink} target="_blank">
                                    Learn more
                                </Link>
                            </div>
                        )}
                    </DialogContent>
                    <DialogActions>
                        <DialogTrigger disableButtonEnhancement>
                            <Button ref={okRef} appearance="primary">
                                {alertOkButton}
                            </Button>
                        </DialogTrigger>
                        {alertCancelButton && (
                            <DialogTrigger disableButtonEnhancement>
                                <Button appearance="secondary">
                                    {alertCancelButton}
                                </Button>
                            </DialogTrigger>
                        )}
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    );
};
