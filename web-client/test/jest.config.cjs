const path = require("path");

/** @type {import('jest').Config} */
module.exports = {
    rootDir: path.resolve(__dirname, "../"),
    testEnvironment: "jsdom",
    setupFilesAfterEnv: [
        "<rootDir>/test/jest.setup.ts"
    ],
    moduleNameMapper: {
        "\\.(css|less|scss|sass)$": "identity-obj-proxy",
        "^ui/(.*)": "<rootDir>/src/ui/$1",
        "^core/(.*)": "<rootDir>/src/core/$1",
        "^low/(.*)": "<rootDir>/src/low/$1",
        "^@test$": "<rootDir>/test"
    },
    transform: {
        "\\.[jt]sx?$": [
            "babel-jest",
            {
                presets: [
                    [
                        "@babel/preset-env",
                        {
                            targets: {
                                node: "current"
                            }
                        }
                    ],
                    "@babel/preset-react",
                    "@babel/preset-typescript",
                ]
            }
        ]
    },
};
