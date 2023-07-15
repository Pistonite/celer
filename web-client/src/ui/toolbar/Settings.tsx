//! Setting control to launch the settings dialog

import React from "react";
import {
    Button,
    Dialog,
    DialogActions,
    DialogBody,
    DialogContent,
    DialogSurface,
    DialogTitle,
    DialogTrigger,
    MenuItem,
    ToolbarButton,
    Tooltip,
} from "@fluentui/react-components";
import { Settings20Regular } from "@fluentui/react-icons";

import { ControlComponentProps, ToolbarControl } from "./util";
import { SettingsDialog } from "./SettingsDialog";

/// The settings control
export const Settings: ToolbarControl = {
    ToolbarButton: React.forwardRef<HTMLButtonElement>((_, ref) => {
        return (
            <SettingsInternal>
                <Tooltip content="Settings" relationship="label">
                    <ToolbarButton icon={<Settings20Regular />} ref={ref} />
                </Tooltip>
            </SettingsInternal>
        );
    }),
    MenuItem: () => {
        return (
            <SettingsInternal>
                <MenuItem icon={<Settings20Regular />}>Settings</MenuItem>
            </SettingsInternal>
        );
    },
};

/// Internal settings dialog component
const SettingsInternal: React.FC<ControlComponentProps> = ({ children }) => {
    // TODO: Implement the settings dialog
    return (
        <Dialog>
            <DialogTrigger disableButtonEnhancement>{children}</DialogTrigger>
            <DialogSurface
                id="settings-dialog-root"
                aria-describedby={undefined}
            >
                <DialogBody>
                    <DialogTitle>Settings</DialogTitle>
                    <DialogContent>
                        <SettingsDialog />
                    </DialogContent>
                    <DialogActions>
                        <DialogTrigger disableButtonEnhancement>
                            <Button appearance="primary">Done</Button>
                        </DialogTrigger>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    );
};
