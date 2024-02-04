//! Control for selecting export options

import { forwardRef } from "react";
import { useSelector, useStore } from "react-redux";
import {
    Body1,
    Menu,
    MenuItem,
    MenuList,
    MenuPopover,
    MenuTrigger,
    Spinner,
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

import { AppStore, documentSelector } from "core/store";
import { ExecDoc, ExpoDoc, ExportIcon, ExportMetadata } from "low/celerc";

import { ControlComponentProps, ToolbarControl } from "./util";
import { Kernel, useKernel } from "core/kernel";
import { errorToString } from "low/utils";

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
                    {exportMetadata?.map((_, i) => (
                        <ExportButton key={i} exportMetadata={exportMetadata} index={i} />
                    ))}
                </MenuList>
            </MenuPopover>
        </Menu>
    );
};

const ExportButton: React.FC<{ exportMetadata: ExportMetadata[], index: number }> = ({ exportMetadata, index }) => {
    const metadata = exportMetadata[index];
    const text = metadata.extension
        ? `${metadata.name} (*.${metadata.extension})`
        : metadata.name;

    const kernel = useKernel();
    const store: AppStore = useStore();
    return (
        <Tooltip
            content={metadata.description}
            relationship="label"
            positioning="after"
        >
            <MenuItem 
                icon={<ExportIconComponent name={metadata.icon} />}
                onClick={() => runExportWizard(exportMetadata, index, kernel, store)}
            >
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

const runExportWizard = async (exportMetadata: ExportMetadata[], index: number, kernel: Kernel, store: AppStore) => {
    let selection = index;
    let error: string | undefined = undefined;
    // eslint-disable-next-line no-constant-condition
    while(true) {
        const ok = await kernel.getAlertMgr().showRich({
            title: "Export",
            component: () => {
                return (
                    <ExportDialog
                        exportMetadata={exportMetadata}
                        initialSelectionIndex={selection}
                        onSelectionChange={(i) => {
                            selection = i;
                        }}
                        error={error}
                    />
                );
            },
            okButton: "Export",
            cancelButton: "Cancel",
        });
        if (!ok) {
            return;
        }
        const result = await kernel.getAlertMgr().showBlocking({
            title: "Export",
            component: () => {
                return (
                    <>
                        <Body1 block>
                            Generating the export file... Download will automatically start once done.
                        </Body1>
                        <Spinner />
                    </>
                );
            }
        }, async (): Promise<ExpoDoc> => {
                return {error: "not implemented"};
            });
        if (result.isOk()) {
            const expoDoc = result.inner();
            if ("success" in expoDoc) {
                const file = expoDoc.success;
                // todo: download file
                console.log(file);
                return;
            } 

            error = expoDoc.error;
        } else {
            const v = result.inner();
            if (v === false) {
                // cancel
                return;
            }
            error = errorToString(v);
        }
    }
}

type ExportDialogProps = {
    /// All available export options
    exportMetadata: ExportMetadata[];
    /// Currently selected export option
    initialSelectionIndex: number;
    onSelectionChange: (index: number) => void;
    /// Error to show
    error?: string;
}
const ExportDialog: React.FC<ExportDialogProps> = ({exportMetadata}) => {
    return (
        <></>
    );
}
