//! Utilities for document

import {
    AppState,
    SettingsState,
    settingsSelector,
    documentSelector,
    viewSelector,
} from "core/store";
import {
    DocPoorText,
    DocRichTextBlock,
    ExecDoc,
    PluginOptions,
} from "low/celerc";
import { shallowArrayEqual } from "low/utils";
import { parseUserConfigOptions } from "./useDocCurrentUserPluginConfig";
import { getSplitExportPluginConfigs } from "./export";

/// Get the previous or next <delta>-th split.
export const getRelativeSplitLocation = (
    doc: ExecDoc,
    section: number,
    line: number,
    delta: number,
    splitTypes: string[],
): { section: number; line: number } => {
    let currentSection = section;
    let currentLine = line;
    const lineDelta = delta > 0 ? 1 : -1;
    let remaining = delta > 0 ? delta : -delta;
    while (remaining !== 0) {
        const newLoc = getRelativeLocation(
            doc,
            currentSection,
            currentLine,
            lineDelta,
        );
        currentSection = newLoc.section;
        currentLine = newLoc.line;

        const line = doc.route[currentSection].lines[currentLine];
        if (!line.counterText || !line.counterText.tag) {
            // the line doesn't have a counter type
            continue;
        }
        const tagName = line.counterText.tag;
        const tag = doc.project.tags[tagName];
        if (!tag || !tag.splitType) {
            // the counter type is invalid or doesn't have a split type
            continue;
        }
        if (splitTypes.includes(tag.splitType)) {
            // found a split line
            remaining -= 1;
        }
    }

    return { section: currentSection, line: currentLine };
};

/// Get the location relative to another location by line delta
///
/// If the new location is out of bound, the first or last line is returned.
/// The return value is always a valid line location
export const getRelativeLocation = (
    doc: ExecDoc,
    section: number,
    line: number,
    delta: number,
): { section: number; line: number } => {
    // Convert to absolute line index
    let absLineIndex = line;
    for (let i = section - 1; i >= 0; i--) {
        absLineIndex += doc.route[i].lines.length;
    }
    // Add delta
    absLineIndex += delta;
    if (absLineIndex <= 0) {
        return { section: 0, line: 0 };
    }
    // Convert back to section and line
    for (let i = 0; i < doc.route.length; i++) {
        if (absLineIndex < doc.route[i].lines.length) {
            return { section: i, line: absLineIndex };
        }
        absLineIndex -= doc.route[i].lines.length;
    }
    // return last line if out of bound
    return {
        section: doc.route.length - 1,
        line: doc.route[doc.route.length - 1].lines.length - 1,
    };
};

/// Function to remove the tag from the text and return the just text content
export const removeTags = (text: Omit<DocRichTextBlock, "tag">[]): string => {
    return text.map(removeTag).join("");
};

export const removeTag = (text: Omit<DocRichTextBlock, "tag">): string => {
    return text.text;
};

/// Return just the text content of poor texts
export const removeLinks = (text: DocPoorText): string => {
    return text.map((t) => t.data).join("");
};

/// Get the default split types for a document defined in the config
export const getDefaultSplitTypes = (doc: ExecDoc): string[] => {
    const splitTags = doc.project.splits;
    const output: string[] = [];
    splitTags.forEach((tag) => {
        const splitType = doc.project.tags[tag]?.splitType;
        if (splitType) {
            output.push(splitType);
        }
    });
    return output;
};

/// Get all split types defined in the document tags
export const getAllSplitTypes = (doc: ExecDoc): string[] => {
    const output = new Set<string>();
    Object.values(doc.project.tags).forEach((tag) => {
        if (tag.splitType) {
            output.add(tag.splitType);
        }
    });
    const array = Array.from(output);
    array.sort();
    return array;
};

const RECOMPILE_ON_SETTINGS: (keyof SettingsState)[] = [
    "compilerEntryPath",
    "enabledAppPlugins",
    "pluginMetadatas",
];

/// If a recompile/reload is needed when state changes
///
/// This is async to batch multiple updates
export function isRecompileNeeded(
    oldState: AppState,
    newState: AppState,
): boolean {
    const { suppressRecompile } = viewSelector(newState);
    if (suppressRecompile) {
        return false;
    }
    const oldSettings = settingsSelector(oldState);
    const newSettings = settingsSelector(newState);
    for (let i = 0; i < RECOMPILE_ON_SETTINGS.length; i++) {
        const key = RECOMPILE_ON_SETTINGS[i];
        if (oldSettings[key] !== newSettings[key]) {
            return true;
        }
    }

    // user plugin config
    if (
        oldSettings.enableUserPlugins !== newSettings.enableUserPlugins ||
        oldSettings.userPluginConfig !== newSettings.userPluginConfig
    ) {
        const newDocument = documentSelector(newState);
        if (newSettings.enableUserPlugins) {
            const { val } = parseUserConfigOptions(
                newSettings.userPluginConfig,
                newDocument.document?.project.title,
            );
            return !!val; // false means error in config
        }
        // user plugin config disabled
        // if old has config, recompile
        const oldDocument = documentSelector(oldState);
        if (oldSettings.enableUserPlugins) {
            const { val } = parseUserConfigOptions(
                oldSettings.userPluginConfig,
                oldDocument.document?.project.title,
            );
            if (val && val.length > 0) {
                return true;
            }
        }
    }

    return false;
}

let lastPluginOptionInputs: unknown[] | undefined = undefined;
let lastPluginOptionResult: PluginOptions | undefined = undefined;

/// Get the raw plugin options to pass to the compiler
export const getRawPluginOptions = (
    state: AppState,
): PluginOptions | undefined => {
    const settings = settingsSelector(state);
    const {
        pluginMetadatas,
        enabledAppPlugins,
        enableUserPlugins,
        userPluginConfig,
    } = settings;
    const { document, serial } = documentSelector(state);

    const currentInputs = [
        pluginMetadatas,
        enabledAppPlugins,
        enableUserPlugins,
        userPluginConfig,
        serial,
    ];
    if (
        lastPluginOptionInputs !== undefined &&
        shallowArrayEqual(currentInputs, lastPluginOptionInputs)
    ) {
        return lastPluginOptionResult;
    }
    lastPluginOptionInputs = currentInputs;
    lastPluginOptionResult = getRawPluginOptionsForTitle(
        settings,
        document?.project.title,
    );
    return lastPluginOptionResult;
};

/// Get the raw plugin options for the specified title
export function getRawPluginOptionsForTitle(
    state: SettingsState,
    title: string | undefined,
): PluginOptions | undefined {
    const {
        pluginMetadatas,
        enabledAppPlugins,
        enableUserPlugins,
        userPluginConfig,
    } = state;

    const metadata = title ? pluginMetadatas[title] || [] : [];
    const routePluginIds = metadata
        .map((m) => {
            if (!m.isFromUser) {
                return m.displayId;
            }
        })
        .filter(Boolean) as string[];

    const add = [];
    if (enabledAppPlugins["export-split"]) {
        getSplitExportPluginConfigs().forEach((config) => add.push(config));
    }
    if (enableUserPlugins) {
        const { val } = parseUserConfigOptions(userPluginConfig, title);
        if (val) {
            add.push(...val);
        }
    }
    const remove = metadata
        .map((m, i) => {
            if (m.isFromUser && !enableUserPlugins) {
                return undefined;
            }
            if (m.isEnabled) {
                return undefined;
            }
            return i;
        })
        .filter((x) => x !== undefined) as number[];
    if (remove.length === 0 && add.length === 0) {
        return undefined;
    }
    return { routePluginIds, remove, add };
}
