import YAML from "js-yaml";

import { ExportMetadata, ExportRequest } from "low/celerc";
import { DocSettingsState } from "./state";
import { Result, allocErr, allocOk } from "low/utils";

/// Get a unique identifier for the export metadata
///
/// Used as the key in config storage
export const getExporterId = (metadata: ExportMetadata): string => {
    return `${metadata.pluginId}${metadata.pluginId.length}${metadata.exportId || ""}`;
}

/// Check if the export needs a config to be set
///
/// An exporter must provide an example config string, if it wants to take config as input
export const isConfigNeeded = (metadata: ExportMetadata): boolean => {
    return !!metadata.exampleConfig;
}

/// Get the config string for the export from settings, or use the default from the metadata
export const getExportConfig = (metadata: ExportMetadata, state: DocSettingsState): string => {
    const id = getExporterId(metadata);
    if (id in state.exportConfigs) {
        return state.exportConfigs[id];
    }
    return metadata.exampleConfig || "";
}

/// Get the display label for the export, with the name and file extension
export const getExportLabel = (metadata: ExportMetadata): string => {
    return metadata.extension
        ? `${metadata.name} (*.${metadata.extension})`
        : metadata.name;
}

export const createExportRequest = (metadata: ExportMetadata, config: string): Result<ExportRequest, unknown> => {
    try {
        const configPayload = YAML.load(config);
        const request: ExportRequest = {
            pluginId: metadata.pluginId,
            exportId: metadata.exportId || "",
            payload: configPayload,
        };
        return allocOk(request);
    } catch (e) {
        return allocErr(e);
    }
}
