import path from "path";
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
        return {
            key: path.join(__dirname, "../cert/cert-key.pem"),
            cert: path.join(__dirname, "../cert/cert.pem"),
        };
    } catch (e) {
        return undefined;
    }
};

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        react(),
        tsconfigPaths(),
        removeRustStyleDocComments(),
        wasm(),
        topLevelAwait(),
    ],
    server: {
        https: createHttpsConfig(),
        proxy: {
            "/docs": {
                target: "http://localhost:3173",
                changeOrigin: false,
            },
            "^/docs/.*": {
                target: "http://localhost:3173",
                changeOrigin: false,
            },
        },
    },
    build: {
        rollupOptions: {
            output: {
                chunkFileNames: (info) => {
                    for (let i = 0; i < info.moduleIds.length; i++) {
                        if (info.moduleIds[i].includes("DocRoot")) {
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
