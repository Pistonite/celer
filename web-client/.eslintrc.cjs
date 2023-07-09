module.exports = {
    env: { browser: true, es2020: true },
    extends: [
        "eslint:recommended",
        "plugin:@typescript-eslint/recommended",
        "plugin:react-hooks/recommended",
    ],
    parser: "@typescript-eslint/parser",
    parserOptions: { ecmaVersion: "latest", sourceType: "module" },
    plugins: ["react-refresh"],
    rules: {
        "react-refresh/only-export-components": "warn",
        "@typescript-eslint/no-unused-vars": [
            "warn",
            {
                varsIgnorePattern: "_",
                argsIgnorePattern: "_",
            },
        ],
        // These are covered by prettier
        // indent: ["warn", 4],
        // quotes: ["warn", "double"],
        // semi: ["warn", "always"],
        "no-multiple-empty-lines": [
            "warn",
            {
                max: 1,
            },
        ],
        "no-console": [
            "warn",
            {
                allow: ["error", "warn", "info"],
            },
        ],
        "no-unreachable-loop": ["error"],
        curly: ["warn", "all"],
    },
};
