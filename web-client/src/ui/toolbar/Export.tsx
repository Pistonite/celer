//! Control for selecting export options

import { forwardRef } from "react";
import { useSelector } from "react-redux";
import {
    Menu,
    MenuItem,
    MenuList,
    MenuPopover,
    MenuTrigger,
    ToolbarButton,
    Tooltip,
} from "@fluentui/react-components";
import {
    AnimalCat20Regular,
    ArrowDownload20Regular,
    Box20Regular,
    Code20Regular,
    Document20Regular,
    DocumentChevronDouble20Regular,
    FolderZip20Regular,
    Image20Regular,
    Video20Regular,
} from "@fluentui/react-icons";

import { documentSelector } from "core/store";
import { ExecDoc, ExportIcon, ExportMetadata } from "low/celerc";

import { ControlComponentProps, ToolbarControl } from "./util";

export const Export: ToolbarControl = {
    ToolbarButton: forwardRef<HTMLButtonElement>((_, ref) => {
        const { enabled, tooltip, exportMetadata } = useExportControl();
        return (
            <ExportInternal exportMetadata={exportMetadata}>
                <Tooltip content={tooltip} relationship="label">
                    <ToolbarButton
                        ref={ref}
                        icon={<ArrowDownload20Regular />}
                        disabled={!enabled}
                    />
                </Tooltip>
            </ExportInternal>
        );
    }),
    MenuItem: () => {
        const { enabled, exportMetadata } = useExportControl();
        return (
            <ExportInternal exportMetadata={exportMetadata}>
                <MenuItem icon={<ArrowDownload20Regular />} disabled={!enabled}>
                    Export
                </MenuItem>
            </ExportInternal>
        );
    },
};

const useExportControl = () => {
    const { document, exportMetadata } = useSelector(documentSelector);
    const enabled =
        document !== undefined &&
        exportMetadata !== undefined &&
        exportMetadata.length > 0;
    return {
        enabled,
        exportMetadata,
        tooltip: getTooltip(document, exportMetadata),
    };
};

const getTooltip = (
    document: ExecDoc | undefined,
    exportMetadata: ExportMetadata[] | undefined,
) => {
    if (!document) {
        return "No document loaded to export";
    }
    if (!exportMetadata || !exportMetadata.length) {
        return "This document has no export options";
    }
    return "Export this document";
};

type ExportInternalProps = ControlComponentProps & {
    exportMetadata: ExportMetadata[] | undefined;
};

const ExportInternal: React.FC<ExportInternalProps> = ({
    children,
    exportMetadata,
}) => {
    return (
        <Menu>
            <MenuTrigger>{children}</MenuTrigger>
            <MenuPopover>
                <MenuList>
                    {exportMetadata?.map((metadata, i) => (
                        <ExportButton key={i} metadata={metadata} />
                    ))}
                </MenuList>
            </MenuPopover>
        </Menu>
    );
};

const ExportButton: React.FC<{ metadata: ExportMetadata }> = ({ metadata }) => {
    const text = metadata.extension
        ? `${metadata.name} (*.${metadata.extension})`
        : metadata.name;
    return (
        <Tooltip
            content={metadata.description}
            relationship="label"
            positioning="after"
        >
            <MenuItem icon={<ExportIconComponent name={metadata.icon} />}>
                {text}
            </MenuItem>
        </Tooltip>
    );
};

const ExportIconComponent: React.FC<{ name: ExportIcon | undefined }> = ({
    name,
}) => {
    switch (name) {
        case "archive":
            return <FolderZip20Regular />;
        case "binary":
            return <Box20Regular />;
        case "cat":
            return <AnimalCat20Regular />;
        case "code":
            return <Code20Regular />;
        case "data":
            return <DocumentChevronDouble20Regular />;
        case "file":
            return <Document20Regular />;
        case "image":
            return <Image20Regular />;
        case "video":
            return <Video20Regular />;
        default:
            return <Document20Regular />;
    }
};

// type ExportDialogProps = {
//     /// All available export options
//     exportMetadata: ExportMetadata[];
//     /// Currently selected export option
//     selectedMetadata: ExportMetadata;
//     setSelectedMetadata: (metadata: ExportMetadata) => void;
// }
// const ExportDialog: React.FC<{metadata: ExportMetadata}> = ({metadata}) => {
//     return (
//         <></>
//     );
// }
