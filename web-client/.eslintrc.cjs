const path = require("path");

module.exports = {
    env: { browser: true, es2020: true },
    extends: [
        "eslint:recommended",
        "plugin:@typescript-eslint/recommended",
        "plugin:react-hooks/recommended",
        "plugin:import/recommended",
        "plugin:import/typescript",
    ],
    parser: "@typescript-eslint/parser",
    parserOptions: { ecmaVersion: "latest", sourceType: "module" },
    plugins: ["react-refresh", "import"],
    settings: {
        "import/resolver": {
            typescript: {},
            node: {
                extensions: [".js", ".jsx", ".ts", ".tsx"],
            },
        },
        "import/external-module-folders": ["node_modules", "src", "libs"],
    },
    ignorePatterns: ["*.d.ts"],
    rules: {
        // TODO #182: will address this later
        "react-refresh/only-export-components": "off",
        "@typescript-eslint/no-unused-vars": [
            "warn",
            {
                varsIgnorePattern: "_",
                argsIgnorePattern: "_",
            },
        ],
        "no-constant-condition": ["error", { checkLoops: false }],
        "no-multiple-empty-lines": [
            "warn",
            {
                max: 1,
            },
        ],
        "no-console": [
            "warn",
            {
                allow: ["error", "warn"],
            },
        ],
        "no-unreachable-loop": ["error"],
        curly: ["warn", "all"],
        "import/default": "off",
        "import/no-named-as-default": "off",
        "import/no-named-as-default-member": "off",
        "import/no-absolute-path": "warn",
        "import/no-useless-path-segments": "warn",
        "import/no-relative-parent-imports": "warn",
        "import/first": "warn",
        "import/no-unresolved": "off", // we don't need eslint to tell us what is resolved or not
        "import/no-restricted-paths": [
            "error",
            {
                basePath: path.resolve(__dirname, "src"),
                zones: [
                    {
                        target: "./core",
                        from: ["./ui"],
                        message:
                            "Cannot import ui from core layer. Please refactor to keep the layers clean.",
                    },
                    {
                        target: "./low",
                        from: ["./ui", "./core"],
                        message:
                            "Cannot import other layers from the low layer. Please refactor to keep the layers clean.",
                    },
                ],
            },
        ],
        "import/order": [
            "warn",
            {
                groups: ["builtin", "external", "internal", "sibling"],
                pathGroups: [
                    {
                        pattern: "leaflet",
                        group: "external",
                    },
                ],
            },
        ],
        "import/no-internal-modules": [
            "warn",
            {
                allow: ["*/*", "leaflet/**/*", "react-grid-layout/**/*"],
            },
        ],
    },
};
