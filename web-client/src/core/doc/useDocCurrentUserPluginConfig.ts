import { useMemo } from "react";
import { useSelector } from "react-redux";
import YAML from "js-yaml";

import { errstr } from "pure/utils";
import { Result, tryCatch } from "pure/result";

import { documentSelector, settingsSelector } from "core/store";
import { Value } from "low/celerc";

/// Hook to get the parsed user config options for the current document
export const useDocCurrentUserPluginConfig = (): Result<Value[], string> => {
    const { document, serial } = useSelector(documentSelector);
    const { userPluginConfig, enableUserPlugins } =
        useSelector(settingsSelector);

    /* eslint-disable react-hooks/exhaustive-deps*/
    const result = useMemo(() => {
        if (!enableUserPlugins) {
            return { val: [] };
        }
        // parse user plugin config
        return parseUserConfigOptions(
            userPluginConfig,
            document?.project.title,
        );
    }, [serial, userPluginConfig, enableUserPlugins]);
    /* eslint-enable react-hooks/exhaustive-deps*/

    return result;
};

export const parseUserConfigOptions = (
    config: string,
    title: string | undefined,
): Result<Value[], string> => {
    const configObjResult = tryCatch(() => YAML.load(config));
    if ("err" in configObjResult) {
        return { err: errstr(configObjResult.err) };
    }
    const configObj = configObjResult.val;
    if (!configObj) {
        return { val: [] };
    }

    if (typeof configObj !== "object") {
        return { err: "Plugin config must be a mapping object!" };
    }

    const options: Value[] = [];
    if ("*" in configObj) {
        const wildcardOptions = configObj["*"];
        if (!Array.isArray(wildcardOptions)) {
            throw new Error("Configuration for `*` must be an array!");
        }
        options.push(...wildcardOptions);
    }

    if (title) {
        if (title in configObj) {
            const docOptions: unknown =
                configObj[title as keyof typeof configObj];
            if (!Array.isArray(docOptions)) {
                return {
                    err: `Configuration for document "${title}" must be an array!`,
                };
            }
            options.push(...docOptions);
        }
    }

    return { val: options };
};
