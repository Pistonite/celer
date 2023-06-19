import { TransformContext, defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Celer Docs",
  description: "Documentation for Celer Route Engine",
  head: [
    // Favicon
    [ 'link', { rel: 'icon', href: '/docs/icon.svg', type: 'image/svg+xml' } ],
    // Open Graph
    [ 'meta', { property: 'og:site_name', content: 'celer.pistonite.org' } ],
    [ 'meta', { property: 'og:type', content: 'website' } ],
    [ 'meta', { property: 'og:image', content: 'https://celer.pistonite.org/docs/icon.png' } ],
    [ 'meta', { property: 'og:description', content: 'Documentation for Celer Route Engine' } ],
    // <meta property="og:title" content="Regen"> dynamic
    // <meta property="og:url" content="https://regen.pistonite.org"> dynamic
  ],
  transformHead: async (context: TransformContext) => {
    const page = context.page === "index.md" ? "" : ("/" + ( context.page.endsWith(".md") ? context.page.slice(0, -3) : context.page ));
    return [
      [ 'meta', { property: 'og:title', content: context.title } ],
      [ 'meta', { property: 'og:url', content: `https://celer.pistonite.org/docs${page}` } ],
    ];
  },
  base: "/docs/",
  cleanUrls: false, // tide has no easy way to support this
  themeConfig: {
    logo: "/icon.svg",
    
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Usage', link: '/' },
      { text: 'Syntax', link: '/markdown-examples' },
      { text: 'Developer', link: '/developer/' },
    ],

    sidebar: {
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
