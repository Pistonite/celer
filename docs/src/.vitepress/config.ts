import { TransformContext, defineConfig } from 'vitepress'
import { writingRoutesNav, writingRoutesSidebar } from './nav'

// https://vitepress.dev/reference/site-config
export default defineConfig({
    title: "Celer Docs",
    description: "Documentation for Celer Route Engine",
    head: [
        // Favicon
        [ 'link', { rel: 'icon', href: '/static/celer-3.svg', type: 'image/svg+xml' } ],
        // Color
        [ 'meta', { property: 'theme-color', content: 'rgb(173,255,184)' } ],
        // Open Graph
        [ 'meta', { property: 'og:site_name', content: 'celer.pistonite.org' } ],
        [ 'meta', { property: 'og:type', content: 'website' } ],
        [ 'meta', { property: 'og:image', content: 'https://celer.pistonite.org/static/celer-3.png' } ],
        [ 'meta', { property: 'og:description', content: 'Documentation for Celer Route Engine' } ],
    ],
    transformHead: async (context: TransformContext) => {
        const page = context.page === "index.md" ? "" : ("/" + ( context.page.endsWith(".md") ? context.page.slice(0, -3) : context.page ));
        return [
            [ 'meta', { property: 'og:title', content: context.title } ],
            [ 'meta', { property: 'og:url', content: `https://celer.pistonite.org/docs${page}` } ],
        ];
    },
    base: "/docs/",
    cleanUrls: true,
    themeConfig: {
        logo: "/icon.svg",

        // https://vitepress.dev/reference/default-theme-config
        nav: [
            { text: 'Home', link: '/' },
            writingRoutesNav,
            { text: 'Writing Plugins', link: '/plugin/' },
            { text: 'Developer', link: '/developer/' },
        ],

        sidebar: {
            ...writingRoutesSidebar,
            "/usage/": [
                {
                    text: 'Examples',
                    items: [
                        { text: 'Markdown Examples', link: '/markdown-examples' },
                        { text: 'Runtime API Examples', link: '/api-examples' }
                    ]
                }
            ],
            "/developer/": [
                {
                    text: 'Web Client',
                    items: [
                        { text: 'core-engine', link: '/developer/web-client/core-engine' },
                        { text: 'Redux Store', link: '/developer/web-client/data-store' },
                    ]
                }
            ]
        },

        socialLinks: [
            { icon: 'github', link: 'https://github.com/Pistonite/celer' }
        ],

        search: {
            provider: "local"
        },

    }
})
