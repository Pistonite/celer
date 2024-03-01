//! Control for selecting export options

import { Buffer } from "buffer/";
import { forwardRef, useState } from "react";
import { useSelector, useStore } from "react-redux";
import {
    Body1,
    Dropdown,
    Field,
    Menu,
    MenuItem,
    MenuList,
    MenuPopover,
    MenuTrigger,
    Spinner,
    ToolbarButton,
    Tooltip,
    Option,
    Divider,
    Link,
    Button,
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
import { ungzip } from "pako";

import { errstr } from "pure/utils";
import { fsSave } from "pure/fs";

import { ErrorBar, PrismEditor } from "ui/shared";
import {
    createExportRequest,
    getExportConfig,
    getExportLabel,
    isConfigNeeded,
} from "core/doc";
import {
    AppStore,
    documentSelector,
    settingsActions,
    settingsSelector,
} from "core/store";
import { Kernel, useKernel } from "core/kernel";
import { ExecDoc, ExpoDoc, ExportIcon, ExportMetadata } from "low/celerc";
import { console } from "low/utils";
import { useActions } from "low/store";

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
                    {exportMetadata?.map((_, i) => (
                        <ExportButton
                            key={i}
                            exportMetadata={exportMetadata}
                            index={i}
                        />
                    ))}
                </MenuList>
            </MenuPopover>
        </Menu>
    );
};

const ExportButton: React.FC<{
    exportMetadata: ExportMetadata[];
    index: number;
}> = ({ exportMetadata, index }) => {
    const metadata = exportMetadata[index];
    const text = getExportLabel(metadata);

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
                onClick={() =>
                    runExportWizard(exportMetadata, index, kernel, store)
                }
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

const runExportWizard = async (
    exportMetadata: ExportMetadata[],
    index: number,
    kernel: Kernel,
    store: AppStore,
) => {
    const state = store.getState();
    let selection = index;
    let error: string | undefined = undefined;
    // show the extra config dialog based on the initial selection
    const enableConfig = isConfigNeeded(exportMetadata[selection]);
    let config: string = getExportConfig(
        exportMetadata[selection],
        settingsSelector(state),
    );
    while (true) {
        // show extra config dialog if needed
        if (enableConfig) {
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
                            config={config}
                            onConfigChange={(c) => {
                                config = c;
                            }}
                        />
                    );
                },
                okButton: "Export",
                cancelButton: "Cancel",
            });
            if (!ok) {
                return;
            }
        }
        // persist the config to settings
        store.dispatch(
            settingsActions.setExportConfig({
                metadata: exportMetadata[selection],
                config,
            }),
        );
        error = await runExportAndShowDialog(
            kernel,
            exportMetadata[selection],
            config,
        );
        if (!error) {
            return;
        }
    }
};

type ExportDialogProps = {
    /// All available export options
    exportMetadata: ExportMetadata[];
    /// Currently selected export option
    initialSelectionIndex: number;
    onSelectionChange: (index: number) => void;
    /// Error to show
    error?: string;
    /// The config string
    config: string;
    onConfigChange: (config: string) => void;
};
const ExportDialog: React.FC<ExportDialogProps> = ({
    exportMetadata,
    initialSelectionIndex,
    onSelectionChange,
    error,
    config,
    onConfigChange,
}) => {
    const { setExportConfigToDefault } = useActions(settingsActions);

    const [selectedIndex, setSelectedIndex] = useState(initialSelectionIndex);
    const [configValue, setConfigValue] = useState(config);

    const metadata = exportMetadata[selectedIndex];
    const enableConfig = isConfigNeeded(metadata);

    return (
        <>
            <Field label="Select the export format" hint={metadata.description}>
                <Dropdown
                    value={getExportLabel(metadata)}
                    selectedOptions={[selectedIndex.toString()]}
                    onOptionSelect={(_, data) => {
                        const index = parseInt(data.selectedOptions[0]);
                        setSelectedIndex(index);
                        onSelectionChange(index);
                    }}
                >
                    {exportMetadata.map((metadata, i) => (
                        <Option key={i} value={i.toString()}>
                            {getExportLabel(metadata)}
                        </Option>
                    ))}
                </Dropdown>
            </Field>
            {enableConfig && (
                <>
                    <Divider style={{ marginTop: 8, marginBottom: 8 }} />
                    <Body1 block style={{ marginBottom: 8 }}>
                        This export option accepts extra configuration.
                        {metadata.learnMore && (
                            <>
                                {" "}
                                <Link href={metadata.learnMore} target="_blank">
                                    Learn more
                                    {!(
                                        metadata.learnMore.startsWith("/") ||
                                        metadata.learnMore.startsWith(
                                            window.location.origin,
                                        )
                                    ) && " (external link)"}
                                </Link>
                            </>
                        )}
                    </Body1>
                    {error && <ErrorBar title="Error">{error}</ErrorBar>}
                    <PrismEditor
                        language="yaml"
                        value={configValue}
                        setValue={(x) => {
                            setConfigValue(x);
                            onConfigChange(x);
                        }}
                    />
                    <Button
                        onClick={() => {
                            setExportConfigToDefault({ metadata });
                            const config = metadata.exampleConfig || "";
                            setConfigValue(config);
                            onConfigChange(config);
                        }}
                    >
                        Reset configuration
                    </Button>
                </>
            )}
        </>
    );
};

/// Run the export and show a blocking dialog
///
/// Return error string or empty string for success
async function runExportAndShowDialog(
    kernel: Kernel,
    metadata: ExportMetadata,
    config: string,
): Promise<string> {
    let cancelled = false;
    const result = await kernel.getAlertMgr().showBlocking(
        {
            title: "Export",
            component: () => {
                return (
                    <>
                        <Body1 block>
                            Generating the export file... Download will
                            automatically start once done.
                        </Body1>
                        <div style={{ padding: 16 }}>
                            <Spinner />
                        </div>
                    </>
                );
            },
        },
        async (): Promise<string> => {
            const request = createExportRequest(metadata, config);
            if ("err" in request) {
                return errstr(request.err);
            }
            const expoDoc = await kernel.export(request.val);
            if (cancelled) {
                return "";
            }
            return downloadExport(expoDoc);
        },
    );
    if ("err" in result) {
        const error = result.err;
        if (error === false) {
            // cancel
            cancelled = true;
            return "";
        }
        return errstr(error);
    }
    return result.val;
}

function downloadExport(expoDoc: ExpoDoc): string {
    if ("success" in expoDoc) {
        const { fileName, fileContent } = expoDoc.success;
        console.info(
            `received exported content with type: ${fileContent.type}`,
        );
        let data: string | Uint8Array;
        switch (fileContent.type) {
            case "text": {
                data = fileContent.data;
                break;
            }
            case "base64": {
                data = Buffer.from(fileContent.data, "base64");
                break;
            }
            case "base64Gzip": {
                const compressed = Buffer.from(fileContent.data, "base64");
                data = ungzip(compressed);
            }
        }
        console.info(`saving file: ${fileName}`);
        fsSave(data, fileName);
        return "";
    }

    return expoDoc.error;
}
