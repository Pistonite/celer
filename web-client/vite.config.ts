import path from "path";
import fs from "fs";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tsconfigPaths from "vite-tsconfig-paths";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

const kebabCase = (x: string) =>
    x.replace(/([a-z])([A-Z])/g, "$1-$2").toLowerCase();

const removeRustStyleDocComments = () => {
    return {
        name: "remove-rust-style-doc-comments",
        transform(code: string, _id: string) {
            return code
                .split("\n")
                .filter((x) => !x.startsWith("//!"))
                .join("\n");
        },
    };
};

const createHttpsConfig = () => {
    try {
        const key = path.join(__dirname, "../cert/cert-key.pem");
        const cert = path.join(__dirname, "../cert/cert.pem");
        if (fs.existsSync(key) && fs.existsSync(cert)) {
            return { key, cert };
        }
    } catch (e) {}
    return undefined;
};

const https = createHttpsConfig();

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        react(),
        tsconfigPaths(
            // projects: ["./tsconfig.json", "../libs/tsconfig.json"],
        ),
        removeRustStyleDocComments(),
        wasm(),
        topLevelAwait(),
    ],
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
});
