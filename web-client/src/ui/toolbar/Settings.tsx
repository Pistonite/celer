//! Setting control to launch the settings dialog

import React, { forwardRef } from "react";
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

import { viewActions } from "core/store";
import { useActions } from "low/store";

import { ControlComponentProps, ToolbarControl } from "./util";
import { SettingsDialog } from "./settings";

/// The settings control
export const Settings: ToolbarControl = {
    ToolbarButton: forwardRef<HTMLButtonElement>((_, ref) => {
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
    const { setEditingKeyBinding } = useActions(viewActions);
    return (
        <Dialog onOpenChange={() => setEditingKeyBinding(undefined)}>
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
