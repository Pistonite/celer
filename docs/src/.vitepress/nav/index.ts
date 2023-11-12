export * from "./route";
export * from "./plugin";

export const homeNav = {
    text: "Home",
    link: "/",
};

export const homeSideBar = {
    "/": [
        {
            text: "Features",
            items: [
                { text: "Layout", link: "/layout" },
                { text: "Toolbar", link: "/toolbar" },
                { text: "Document", link: "/doc" },
                { text: "Map", link: "/map" },
            ],
        },
        {
            text: "Misc",
            items: [{ text: "Browser Support", link: "/browser-support" }],
        },
    ],
};
