
import "./styles/index.css"
import FoldersPage from "./components/Folders"
import FileInfo from "./components/FileInfo";
import { useState } from "react";


function App() {
  const [screen, setScreen] =  useState("folders");
  const [folder, setFolderPath] =  useState("folders");
  const handleSelectFolder = (folderPath: string) => {
    console.log("clicked from " + folderPath)
    setScreen("media");
    setFolderPath(folderPath)
  }

  switch(screen){
    case 'folders':
      return (
        <FoldersPage onSelectFolder={handleSelectFolder}/>
      );
      break;
    case 'media':
      return (
        <FileInfo folderName={folder}/>
      );
      break;
    default:
      return (<FoldersPage onSelectFolder={handleSelectFolder}/>)
      break;
  }
}

export default App
