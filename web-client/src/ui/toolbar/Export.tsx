//! Control for selecting export options

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

import { AppStore, documentSelector, settingsActions, settingsSelector } from "core/store";
import { ExecDoc, ExpoDoc, ExportIcon, ExportMetadata } from "low/celerc";

import { ControlComponentProps, ToolbarControl } from "./util";
import { Kernel, useKernel } from "core/kernel";
import { errorToString, sleep } from "low/utils";
import { createExportRequest, getExportConfig, getExportLabel, isConfigNeeded } from "core/doc";
import { ErrorBar, PrismEditor } from "ui/shared";
import { useActions } from "low/store";

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
    const state = store.getState();
    let selection = index;
    let error: string | undefined = undefined;
    // show the extra config dialog based on the initial selection
    const enableConfig = isConfigNeeded(exportMetadata[selection]);
    let config: string = getExportConfig(exportMetadata[selection], settingsSelector(state));
    // eslint-disable-next-line no-constant-condition
    while(true) {
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
        store.dispatch(settingsActions.setExportConfig({metadata: exportMetadata[selection], config}));
        const result = await kernel.getAlertMgr().showBlocking({
            title: "Export",
            component: () => {
                return (
                    <>
                        <Body1 block>
                            Generating the export file... Download will automatically start once done.
                        </Body1>
                        <div style={{padding: 16}}>
                        <Spinner />
                        </div>
                    </>
                );
            }
        }, async (): Promise<ExpoDoc> => {
                const requestResult = createExportRequest(exportMetadata[selection], config);
                if (requestResult.isErr()) {
                    return {error: errorToString(requestResult.inner()) };
                }
                const request = requestResult.inner();
                return await kernel.export(request);
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
    /// The config string
    config: string;
    onConfigChange: (config: string) => void;
}
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
            <Field
                label="Select the export format"
                hint={metadata.description}
            >
                <Dropdown
                    value={getExportLabel(metadata)}
                    selectedOptions={[selectedIndex.toString()]}
                    onOptionSelect={(_, data) => {
                        const index = parseInt(data.selectedOptions[0]);
                        setSelectedIndex(index);
                        onSelectionChange(index);
                    }}
                >
                    {
                        exportMetadata.map((metadata, i) => (
                            <Option key={i} value={i.toString()} >
                                {getExportLabel(metadata)}
                            </Option>
                        ))
                    }
                </Dropdown>
            </Field>
            {enableConfig &&
                <>
                    <Divider style={{marginTop: 8, marginBottom: 8}} />
                    <Body1 block style={{marginBottom: 8}}>
                        This export option accepts extra configuration.
                        {metadata.learnMore &&
                            <>
                                {" "}
                                <Link
                                    href={metadata.learnMore}
                                    target="_blank"
                                >
                                    Learn more
                                    {
                                        !(metadata.learnMore.startsWith("/") || metadata.learnMore.startsWith(window.location.origin)) && " (external link)"
                                    }
                                </Link>
                            </>
                        }
                    </Body1>
                    { error && 
                            <ErrorBar title="Error">
                                {error}
                            </ErrorBar>
                    }
                    <PrismEditor
                        language="yaml"
                        value={configValue}
                        setValue={(x) => {
                            setConfigValue(x);
                            onConfigChange(x);
                        }}
                    />
                    <Button onClick={() => {
                        setExportConfigToDefault({metadata});
                        const config = metadata.exampleConfig || "";
                        setConfigValue(config);
                        onConfigChange(config);
                    }}>
                        Reset configuration
                    </Button>
                </>
            }
        </>
    );
}
