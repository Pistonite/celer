export const pluginsNav = {
    text: "Plugins",
    link: "/plugin/",
};

export const pluginsSideBar = {
    "/plugin/": [
        {
            text: "General",
            items: [
                { text: "Getting Started", link: "/plugin/getting-started" },
                { text: "Built-in Plugins", link: "/plugin/built-in" },
                { text: "Settings", link: "/plugin/settings" },
            ],
        },
        {
            text: "Built-in References",
            items: [
                { text: "Link", link: "/plugin/link" },
                { text: "Variables", link: "/plugin/variables" },
                // { text: "Assertion", link: "/plugin/assertion" },
                // { text: "Compatibility", link: "/plugin/compat" },
            ],
        },
        {
            text: "Development",
            items: [
                { text: "TODO", link: "/plugin/getting-started" },
                // { text: "Tags", link: "/route/config/tags" },
                // { text: "Presets", link: "/route/config/presets" },
                // { text: "Map", link: "/route/config/map" },
                // { text: "Other", link: "/route/config/other" },
            ],
        },
    ],
};
