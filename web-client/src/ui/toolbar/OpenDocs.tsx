//! Control to open documentation page

import { MenuItem, ToolbarButton, Tooltip } from "@fluentui/react-components";
import { BookQuestionMark20Regular } from "@fluentui/react-icons";
import { forwardRef } from "react";

import type { ToolbarControl } from "./util";

export const OpenDocs: ToolbarControl = {
    ToolbarButton: forwardRef<HTMLButtonElement>((_, ref) => {
        return (
            <Tooltip content="Help" relationship="label">
                <ToolbarButton
                    ref={ref}
                    icon={<BookQuestionMark20Regular />}
                    onClick={openDocs}
                />
            </Tooltip>
        );
    }),
    MenuItem: () => {
        return (
            <Tooltip
                content="Open documentation in a new tab"
                relationship="label"
            >
                <MenuItem
                    icon={<BookQuestionMark20Regular />}
                    onClick={openDocs}
                >
                    Help
                </MenuItem>
            </Tooltip>
        );
    },
};

const openDocs = () => {
    const a = document.createElement("a");
    a.target = "_blank";
    a.href = "/docs";
    a.click();
};
