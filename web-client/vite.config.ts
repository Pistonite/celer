/// <reference types="vitest/config" />
// @ts-expect-error @types/node
import path from "path";
// @ts-expect-error @types/node
import fs from "fs";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tsconfigPaths from "vite-tsconfig-paths";
import wasm from "vite-plugin-wasm";

declare const __dirname: string;

const createHttpsConfig = () => {
    try {
        const key = path.join(__dirname, "../cert/cert-key.pem");
        const cert = path.join(__dirname, "../cert/cert.pem");
        if (fs.existsSync(key) && fs.existsSync(cert)) {
            return { key, cert };
        }
    } catch (e) {
        // ignore
    }
    return undefined;
};

const https = createHttpsConfig();

const kebabCase = (x: string) =>
    x.replace(/([a-z])([A-Z])/g, "$1-$2").toLowerCase();

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [react(), tsconfigPaths(), wasm()],
    server: {
        https,
        proxy: {
            "/docs": {
                target: "http://localhost:3173",
                changeOrigin: false,
            },
            "^/docs/.*": {
                target: "http://localhost:3173",
                changeOrigin: false,
            },
            "^/api/.*": {
                target: (https ? "https" : "http") + "://0.0.0.0:8173",
                secure: false,
                changeOrigin: false,
            },
        },
    },
    build: {
        rollupOptions: {
            output: {
                chunkFileNames: (info) => {
                    for (let i = 0; i < info.moduleIds.length; i++) {
                        if (info.moduleIds[i].includes("DocController")) {
                            return "assets/doc-[hash].js";
                        }
                        if (info.moduleIds[i].includes("MapRoot")) {
                            return "assets/map-[hash].js";
                        }
                        if (info.moduleIds[i].includes("EditorKernel")) {
                            return "assets/editor-[hash].js";
                        }
                    }
                    const name = kebabCase(info.name);
                    return name.startsWith("assets/")
                        ? `${name}-[hash].js`
                        : `assets/${name}-[hash].js`;
                },
                manualChunks: {
                    "assets/react": ["react", "react-dom"],
                    "assets/fluentui": ["@fluentui/react-components"],
                    "assets/monaco": ["monaco-editor"],
                },
            },
        },
    },
    test: {
        environment: "jsdom",
    },
});
