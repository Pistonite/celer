export const writingRoutesNav = {
    text: "Writing Routes",
    link: "/route/",
};

export const writingRoutesSidebar = {
    "/route/": [
        {
            text: "Using the Editor",
            items: [
                {
                    text: "Getting Started",
                    link: "/route/editor/getting-started",
                },
                { text: "Web Editor", link: "/route/editor/web" },
                { text: "External Editor", link: "/route/editor/external" },
                { text: "Advanced", link: "/route/editor/advanced" },
            ],
        },
        {
            text: "Make a Route",
            items: [
                { text: "YAML Basics", link: "/route/yaml-basics" },
                { text: "Hello, World!", link: "/route/hello-world" },
                { text: "Configuration", link: "/route/configuration" },
                { text: "File Structure", link: "/route/file-structure" },
                { text: "Route Structure", link: "/route/route-structure" },
                { text: "Text and Notes", link: "/route/text-and-notes" },
                { text: "Tagging Text", link: "/route/tagging-text" },
                { text: "Adding Icons", link: "/route/adding-icons" },
                { text: "Counter and Splits", link: "/route/counter-and-splits" },
                { text: "Movements", link: "/route/customizing-movements" },
                // { text: "Customizing Lines", link: "/route/customizing-lines" },
                // {
                //     text: "Customizing Movements",
                //     link: "/route/customizing-movements",
                // },
                { text: "Using Presets", link: "/route/using-presets" },
                { text: "Publish the Route", link: "/route/publish" },
                { text: "Property Reference", link: "/route/property-reference" },
            ],
        },
        {
            text: "Configuration Reference",
            items: [
                { text: "Icons", link: "/route/config/icons" },
                { text: "Tags", link: "/route/config/tags" },
                { text: "Presets", link: "/route/config/presets" },
                { text: "Map", link: "/route/config/map" },
                { text: "Plugins", link: "/plugin/getting-started" },
                { text: "Other", link: "/route/config/other" },
            ],
        },
    ],
};
