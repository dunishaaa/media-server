import { createRoot } from 'react-dom/client'
import App from './App.tsx'

createRoot(document.getElementById('root')!).render(
    <>
        <link rel="icon" type="image/svg+xml" href="./assets/react.svg"/>
        <title> Nubecita </title>
        <App />
    </>
)
