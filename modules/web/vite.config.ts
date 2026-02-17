import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'

const apiTarget = process.env.API_URL || 'http://localhost:4000'

export default defineConfig({
  plugins: [tailwindcss(), react()],
  server: {
    port: 3000,
    host: true,
    proxy: {
      '/graphql': apiTarget,
      '/graphiql': apiTarget,
    },
  },
})
