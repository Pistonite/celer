//! Control for viewing and goto diagnostics
//!
//! Diagnostics are extracted from the document and cached

import { forwardRef, memo } from "react";
import {
    Tooltip,
    Menu,
    MenuItem,
    MenuList,
    MenuPopover,
    MenuTrigger,
    ToolbarButton,
} from "@fluentui/react-components";
import {
    DocumentError20Regular,
    DocumentCheckmark20Regular,
} from "@fluentui/react-icons";
import type { DiagnosticSection } from "core/doc";
import { removeLinks, useDocDiagnostics } from "core/doc";
import { useActions } from "low/store";
import { viewActions } from "core/store";

import type { ControlComponentProps, ToolbarControl } from "./util";

export const ViewDiagnostics: ToolbarControl = {
    ToolbarButton: forwardRef<HTMLButtonElement>((_, ref) => {
        const diagnostics = useDocDiagnostics();
        const { setDocLocation } = useActions(viewActions);
        return (
            <ViewDiagnosticInternal
                data={diagnostics}
                gotoDiagnostic={(section, line) => {
                    setDocLocation({ section, line });
                }}
            >
                <Tooltip
                    content={getControlName(diagnostics.length > 0)}
                    relationship="label"
                >
                    <ToolbarButton
                        ref={ref}
                        disabled={diagnostics.length === 0}
                        icon={
                            diagnostics.length > 0 ? (
                                <DocumentError20Regular />
                            ) : (
                                <DocumentCheckmark20Regular />
                            )
                        }
                    />
                </Tooltip>
            </ViewDiagnosticInternal>
        );
    }),
    MenuItem: () => {
        const diagnostics = useDocDiagnostics();
        const { setDocLocation } = useActions(viewActions);
        return (
            <ViewDiagnosticInternal
                data={diagnostics}
                gotoDiagnostic={(section, line) => {
                    setDocLocation({ section, line });
                }}
            >
                <MenuItem
                    icon={
                        diagnostics.length > 0 ? (
                            <DocumentError20Regular />
                        ) : (
                            <DocumentCheckmark20Regular />
                        )
                    }
                    disabled={diagnostics.length === 0}
                >
                    {getControlName(diagnostics.length > 0)}
                </MenuItem>
            </ViewDiagnosticInternal>
        );
    },
};

const getControlName = (enabled: boolean) =>
    enabled ? "View diagnostics" : "No error";

/// Internal component implementation
type ViewDiagnosticInternalProps = ControlComponentProps & {
    /// Diagnostic data
    data: DiagnosticSection[];
    /// Callback to go to a diagnostic
    gotoDiagnostic: (sectionIndex: number, lineIndex: number) => void;
};
const ViewDiagnosticInternal = memo(
    ({ children, data, gotoDiagnostic }: ViewDiagnosticInternalProps) => {
        return (
            <Menu>
                <MenuTrigger>{children}</MenuTrigger>
                <MenuPopover>
                    <MenuList>
                        {data.map((section, i) => (
                            <Menu key={i}>
                                <MenuTrigger>
                                    <MenuItem>{section.sectionName}</MenuItem>
                                </MenuTrigger>
                                <MenuPopover>
                                    <MenuList>
                                        {section.diagnostics.map((d, i) => (
                                            <Tooltip
                                                key={i}
                                                content={removeLinks(d.msg)}
                                                relationship="description"
                                            >
                                                <MenuItem
                                                    onClick={() => {
                                                        gotoDiagnostic(
                                                            d.sectionIndex,
                                                            d.lineIndex,
                                                        );
                                                    }}
                                                >
                                                    ({d.sectionIndex}-
                                                    {d.lineIndex}) {d.type}:{" "}
                                                    {d.source}
                                                </MenuItem>
                                            </Tooltip>
                                        ))}
                                    </MenuList>
                                </MenuPopover>
                            </Menu>
                        ))}
                    </MenuList>
                </MenuPopover>
            </Menu>
        );
    },
);
