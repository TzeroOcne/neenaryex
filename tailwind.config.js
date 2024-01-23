/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './src/**/*.{html,js,svelte,ts}',
    './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}',
    './node_modules/flowbite-svelte-icons/**/*.{html,js,svelte,ts}',
  ],
  theme: {
    extend: {
      colors: {
        nnry: {
          100: '#240f33',
          200: '#36174d',
          300: '#481f66',
          400: '#5a2680',
          500: '#6c2e99',
          600: '#7e36b3',
          700: '#903dcc',
          800: '#a345e6',
          900: '#b54eff',
        },
        nnrygray: {
          100: '#7f7f7f',
          200: '#6e6e6e',
          300: '#5e5e5e',
          400: '#4e4e4e',
          500: '#3e3e3e',
          600: '#2d2d2d',
          700: '#1d1d1d',
          800: '#0d0d0d',
          900: '#070707',
        },
      },
    },
  },
  darkMode: 'class',
  plugins: [require('flowbite/plugin')],
};

