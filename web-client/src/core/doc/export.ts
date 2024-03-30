import YAML from "js-yaml";

import { Result, tryCatch } from "pure/result";

import { AppState, documentSelector, settingsSelector } from "core/store";
import type { ExportMetadata, ExportRequest, Value } from "low/celerc";
import { consoleDoc as console } from "low/utils";

import { DocSettingsState } from "./state";
import { getDefaultSplitTypes } from "./utils";

/// Get a unique identifier for the export metadata
///
/// Used as the key in config storage
export function getExporterId(metadata: ExportMetadata): string {
    return `${metadata.pluginId}${metadata.pluginId.length}${
        metadata.exportId || ""
    }`;
}

/// Check if the export needs a config to be set
///
/// An exporter must provide an example config string, if it wants to take config as input
export function isConfigNeeded(metadata: ExportMetadata): boolean {
    return !!metadata.exampleConfig;
}

/// Get the config string for the export from settings, or use the default from the metadata
export function getExportConfig(
    metadata: ExportMetadata,
    state: DocSettingsState,
): string {
    const id = getExporterId(metadata);
    if (id in state.exportConfigs) {
        return state.exportConfigs[id];
    }
    return metadata.exampleConfig || "";
}

/// Get the display label for the export, with the name and file extension
export function getExportLabel(metadata: ExportMetadata): string {
    return metadata.extension
        ? `${metadata.name} (*.${metadata.extension})`
        : metadata.name;
}

export function createExportRequest(
    metadata: ExportMetadata,
    config: string,
): Result<ExportRequest, unknown> {
    return tryCatch(() => {
        const configPayload = YAML.load(config);
        const request: ExportRequest = {
            pluginId: metadata.pluginId,
            exportId: metadata.exportId || "",
            payload: configPayload as Value,
        };
        return request;
    });
}

/// Get the plugin configs when the "Export Split" option is enabled
export function getSplitExportPluginConfigs() {
    return [{ use: "export-livesplit" }, { use: "export-mist" }];
}

export function injectSplitTypesIntoRequest(
    request: ExportRequest,
    state: AppState,
) {
    const splitExportConfigs = getSplitExportPluginConfigs();
    if (!splitExportConfigs.find((c) => c.use === request.pluginId)) {
        // not a split export plugin, don't inject splits
        return;
    }
    if (!request.payload || typeof request.payload !== "object") {
        // no payload to inject into
        return;
    }
    const payload = request.payload as Record<string, unknown>;
    if (payload["split-types"]) {
        // already has data, don't override
        return;
    }
    const { splitTypes } = settingsSelector(state);
    let injected: string[];
    if (splitTypes) {
        injected = splitTypes;
    } else {
        const { document } = documentSelector(state);
        if (document) {
            injected = getDefaultSplitTypes(document);
        } else {
            injected = [];
        }
    }
    payload["split-types"] = injected;
    console.info(
        `injected ${injected.length} split types into export request payload.`,
    );
}
