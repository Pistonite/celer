import { useMemo } from "react";
import { useSelector } from "react-redux";
import YAML from "js-yaml";

import { documentSelector, settingsSelector } from "core/store";
import { ExecDoc, Value } from "low/celerc";
import { errorToString } from "low/utils";

type UserPluginOptionsResult = [Value[], undefined] | [undefined, string];

/// Hook to get the parsed user config options for the current document
export const useDocCurrentUserPluginConfig = (): UserPluginOptionsResult => {
    const { document, serial } = useSelector(documentSelector);
    const { userPluginConfig, enableUserPlugins } =
        useSelector(settingsSelector);

    /* eslint-disable react-hooks/exhaustive-deps*/
    const result: UserPluginOptionsResult = useMemo(() => {
        if (!enableUserPlugins) {
            return [[], undefined];
        }
        // parse user plugin config
        return parseUserConfigOptions(userPluginConfig, document);
    }, [serial, userPluginConfig, enableUserPlugins]);
    /* eslint-enable react-hooks/exhaustive-deps*/

    return result;
};

export const parseUserConfigOptions = (
    config: string,
    document: ExecDoc | undefined,
): UserPluginOptionsResult => {
    try {
        const configObj = YAML.load(config);
        if (!configObj) {
            return [[], undefined];
        }
        if (typeof configObj !== "object") {
            throw new Error("Plugin config must be a mapping object!");
        }
        const options: Value[] = [];
        if ("*" in configObj) {
            const wildcardOptions = configObj["*"];
            if (!Array.isArray(wildcardOptions)) {
                throw new Error("Configuration for `*` must be an array!");
            }
            options.push(...wildcardOptions);
        }
        if (document) {
            const title = document.project.title;
            if (title in configObj) {
                const docOptions: unknown =
                    configObj[title as keyof typeof configObj];
                if (!Array.isArray(docOptions)) {
                    throw new Error(
                        `Configuration for document "${title}" must be an array!`,
                    );
                }
                options.push(...docOptions);
            }
        }
        return [options, undefined];
    } catch (e) {
        return [undefined, errorToString(e)];
    }
};
