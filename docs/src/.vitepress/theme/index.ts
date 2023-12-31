// https://vitepress.dev/guide/custom-theme
import { h } from "vue";
import Theme from "vitepress/theme";
import "./style.css";
import SourceLink from "./SourceLink.vue";
import FluentIcon from "./FluentIcon.vue";

export default {
    ...Theme,
    Layout: () => {
        return h(Theme.Layout, null, {
            // https://vitepress.dev/guide/extending-default-theme#layout-slots
        });
    },
    enhanceApp({ app, router, siteData }) {
        app.component("SourceLink", SourceLink);
        app.component("FluentIcon", FluentIcon);
    },
};
