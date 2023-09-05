// @ts-check
import { join } from 'path';

// 1. Import the Skeleton plugin
import { skeleton } from '@skeletonlabs/tw-plugin';
import { tealLightning } from './tealLightning';
import { crimson } from './crimson';

/** @type {import('tailwindcss').Config} */
export default {
  // 2. Opt for dark mode to be handled via the class method
  darkMode: 'class',
  mode: 'jit',
  content: [
    './src/**/*.{html,js,svelte,ts}',
    // 3. Append the path to the Skeleton package
    join(require.resolve(
      '@skeletonlabs/skeleton'),
      '../**/*.{html,js,svelte,ts}'
    )
  ],
  theme: {
    extend: {},
  },
  plugins: [
    // 4. Append the Skeleton plugin (after other plugins)
    require('@tailwindcss/forms'),
    skeleton({
      themes: {
        preset: [{
          name: 'modern', enhancements: true,
        },
        {
          name: 'wintry', enhancements: true,
        },
        {
          name: 'vintage', enhancements: true,
        },
        ],
        custom: [
          tealLightning,
          crimson
        ]
      }
    })
  ]
}
