import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import VfxUpdaterPage from './modules/vfx-updater/VfxUpdaterPage'

// Disable default browser context menu
document.addEventListener('contextmenu', (e) => {
  e.preventDefault();
});

const rootElement = document.getElementById('root')

if (!rootElement) {
  throw new Error('Root element #root not found')
}

// Check if this is a secondary window (VFX updater, etc.)
const isVfxWindow = window.location.href.includes('vfx-updater');

ReactDOM.createRoot(rootElement).render(
  <React.StrictMode>
    {isVfxWindow ? <VfxUpdaterPage /> : <App />}
  </React.StrictMode>,
)
