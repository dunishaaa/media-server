
import { BrowserRouter, Route, Routes} from "react-router-dom";
import "./styles/index.css"
import { lazy, Suspense} from "react";

const FoldersPage = lazy(() => import("./components/Folders"));
const FileInfo = lazy(() => import("./components/FileInfo"));
const DownloadPage = lazy(() => import("./components/Download"));

function App() {
  return (
    <BrowserRouter>
      <Suspense fallback={<div className="loading"> Loading...</div>}>
        <Routes>
          <Route path="/" element={<FoldersPage />} />
          <Route path="/media/:folderPath" element={<FileInfo/>} />
          <Route path="/download" element={<DownloadPage/>} />
          <Route path="*" element={<div>Page not found</div>} />
        </Routes>
      </Suspense>
    </BrowserRouter>
  );
}

export default App
