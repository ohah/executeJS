import * as path from 'node:path';
import { liveDemoPluginRspress } from '@live-demo/plugin-rspress';
import { defineConfig } from 'rspress/config';

export default defineConfig({
  // NOTE: docs 루트에 있으면 rspress 버그가 있어 docs/docs로 유지
  root: path.join(__dirname, 'docs'),

  plugins: [
    liveDemoPluginRspress({
      includeModules: [],
      ui: {
        resizablePanels: {
          autoSaveId: 'executejs-docs',
          defaultPanelSizes: { editor: 55, preview: 45 },
        },
      },
    }),
  ],

  title: 'ExecuteJS',

  markdown: {
    checkDeadLinks: true,
  },
  themeConfig: {
    enableScrollToTop: true,
    socialLinks: [
      {
        icon: 'github',
        mode: 'link',
        content: 'https://github.com/ohah/executeJS',
      },
    ],
  },
  base: '/executeJS/',
  route: {
    cleanUrls: true,
  },
  builderConfig: {
    server: {
      port: 8080,
    },
  },
});
