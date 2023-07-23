import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tsconfigPaths from "vite-tsconfig-paths";

const kebabCase = (x: string) => x.replace(/([a-z])([A-Z])/g, "$1-$2").toLowerCase();

const removeRustStyleDocComments = () => {
    return {
        name: "remove-rust-style-doc-comments",
        transform(code: string, _id: string) {
            return code.split("\n").filter(x => !x.startsWith("//!")).join("\n");
        }
    };
}
// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        react(), 
        tsconfigPaths(),
        removeRustStyleDocComments(),
    ],
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
                    }
                    const name = kebabCase(info.name);
                    return name.startsWith("assets/") ? `${name}-[hash].js` : `assets/${name}-[hash].js`;
                },
                manualChunks: {
                    "assets/react": ["react", "react-dom"],
                    "assets/fluentui": ["@fluentui/react-components"],
                }
            }
        }
    },
});
