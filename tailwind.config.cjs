/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./index.html', './src/**/*.{svelte,ts}'],
  theme: {
    extend: {
      colors: {
        ink: {
          950: '#050816',
          900: '#091121',
          800: '#0f1a2c'
        },
        accent: {
          400: '#67e8f9',
          500: '#2dd4bf',
          600: '#0ea5e9'
        }
      },
      boxShadow: {
        glow: '0 0 0 1px rgba(103,232,249,0.15), 0 16px 40px rgba(15,23,42,0.45)'
      },
      backgroundImage: {
        'mesh-gradient':
          'radial-gradient(circle at top left, rgba(45,212,191,0.18), transparent 30%), radial-gradient(circle at top right, rgba(14,165,233,0.16), transparent 35%), linear-gradient(180deg, rgba(5,8,22,1) 0%, rgba(9,17,33,1) 100%)'
      }
    }
  },
  plugins: []
};
