import { TransformContext, defineConfig } from "vitepress";
import {
    homeNav,
    homeSideBar,
    apiNav,
    apiSideBar,
    pluginsNav,
    pluginsSideBar,
    writingRoutesNav,
    writingRoutesSidebar,
} from "./nav";

// https://vitepress.dev/reference/site-config
export default defineConfig({
    title: "Celer Docs",
    description: "Documentation for Celer Route Engine",
    head: [
        // Favicon
        [
            "link",
            { rel: "icon", href: "/static/celer-3.svg", type: "image/svg+xml" },
        ],
        // Color
        ["meta", { property: "theme-color", content: "rgb(173,255,184)" }],
        // Open Graph
        [
            "meta",
            { property: "og:site_name", content: "celer.placeholder.domain" },
        ],
        ["meta", { property: "og:type", content: "website" }],
        [
            "meta",
            {
                property: "og:image",
                content: "scheme://celer.placeholder.domain/static/celer-3.png",
            },
        ],
        [
            "meta",
            {
                property: "og:description",
                content: "Documentation for Celer Route Engine",
            },
        ],
        [
            "script",
            {
                src: "/docs/transform-fluent-icon.js",
            },
        ],
    ],
    transformHead: async (context: TransformContext) => {
        const page =
            context.page === "index.md"
                ? ""
                : "/" +
                  (context.page.endsWith(".md")
                      ? context.page.slice(0, -3)
                      : context.page);
        return [
            ["meta", { property: "og:title", content: context.title }],
            [
                "meta",
                {
                    property: "og:url",
                    content: `scheme://celer.placeholder.domain/docs${page}`,
                },
            ],
        ];
    },
    base: "/docs/",
    cleanUrls: true,
    themeConfig: {
        // this will force vitepress to generate code that loads the logo from /static
        logo: "/../static/celer-3.svg",

        // https://vitepress.dev/reference/default-theme-config
        nav: [homeNav, writingRoutesNav, pluginsNav, apiNav],

        sidebar: {
            ...homeSideBar,
            ...writingRoutesSidebar,
            ...pluginsSideBar,
            ...apiSideBar,
        },

        socialLinks: [
            { icon: "github", link: "https://github.com/Pistonite/celer" },
        ],

        search: {
            provider: "local",
        },
    },
});
