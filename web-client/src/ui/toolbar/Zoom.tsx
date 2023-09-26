//! Zoom in/out control

import { forwardRef } from "react";
import { MenuItem, ToolbarButton, Tooltip } from "@fluentui/react-components";
import { ZoomIn20Regular, ZoomOut20Regular } from "@fluentui/react-icons";

import { useZoomControl } from "./useZoomControl";
import { ToolbarControl } from "./util";

/// Factory function for zoom controls
const createZoomControl = (isZoomIn: boolean): ToolbarControl => {
    const text = isZoomIn ? "Zoom in" : "Zoom out";
    return {
        ToolbarButton: forwardRef<HTMLButtonElement>((_, ref) => {
            const handler = useZoomControl(isZoomIn);
            return (
                <Tooltip content={text} relationship="label">
                    <ToolbarButton
                        ref={ref}
                        icon={
                            isZoomIn ? (
                                <ZoomIn20Regular />
                            ) : (
                                <ZoomOut20Regular />
                            )
                        }
                        disabled={!handler}
                        onClick={handler}
                    />
                </Tooltip>
            );
        }),
        MenuItem: () => {
            const handler = useZoomControl(isZoomIn);
            console.log("hi");
            return (
                <MenuItem
                    icon={isZoomIn ? <ZoomIn20Regular /> : <ZoomOut20Regular />}
                    disabled={!handler}
                    onClick={handler}
                >
                    {text}
                </MenuItem>
            );
        },
    };
};

/// Zoom in
export const ZoomIn = createZoomControl(true);
/// Zoom out
export const ZoomOut = createZoomControl(false);
