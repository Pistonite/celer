//! Setting control to launch the settings dialog

import React from "react";
import { Button, Dialog, DialogActions, DialogBody, DialogContent, DialogSurface, DialogTitle, DialogTrigger, MenuItem, ToolbarButton, Tooltip } from "@fluentui/react-components";
import { Settings20Regular } from "@fluentui/react-icons";

import { ControlComponentProps, ToolbarControl } from "./util";

/// The settings control
export const Setting: ToolbarControl = {
    ToolbarButton: React.forwardRef<HTMLButtonElement>((_, ref) => {
        return (
            <SettingInternal>
                <Tooltip content="Settings" relationship="label">
                    <ToolbarButton icon={<Settings20Regular />} ref={ref}/>
                </Tooltip>
            </SettingInternal>
        );
    }),
    MenuItem: () => {
        return (
            <SettingInternal>
                <MenuItem icon={<Settings20Regular />}>Settings</MenuItem>
            </SettingInternal>
        );
    }
};

/// Internal settings dialog component
const SettingInternal: React.FC<ControlComponentProps> = ({ children }) => {
    // TODO: Implement the settings dialog
    return (
        <Dialog>
            <DialogTrigger disableButtonEnhancement>
                {children}
            </DialogTrigger>
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>Dialog title</DialogTitle>
                    <DialogContent>
                        Lorem ipsum dolor sit amet consectetur adipisicing elit. Quisquam
                        exercitationem cumque repellendus eaque est dolor eius expedita
                        nulla ullam? Tenetur reprehenderit aut voluptatum impedit voluptates
                        in natus iure cumque eaque?
                    </DialogContent>
                    <DialogActions>
                        <DialogTrigger disableButtonEnhancement>
                            <Button appearance="secondary">Close</Button>
                        </DialogTrigger>
                        <Button appearance="primary">Do Something</Button>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    );
};
