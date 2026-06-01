import { useEffect, useState } from "react";
import "../styles/folder.css"

interface Folders{
  names: string[]
}
function FolderBox(props: { folderName: string, onSelectFolder: (folderName: string) => void}){
    return (
    <div className="box-container" onClick={() => props.onSelectFolder(props.folderName)}>
        <h2>{props.folderName}</h2>
    </div>
    );
}

function FolderGrid(props: {onSelectFolder: (folderName: string) => void}){
  const [folders, setFolders] = useState<Folders>();
  const [_, setLoading] = useState(true);

  useEffect(() => {
    async function fetchFolders() {
      try{
        const response = await fetch("http://192.168.1.80:3000/api/folders");

        if(!response.ok){
          throw new Error("Faled to fetch folders");
        }
        const data: Folders = await response.json();
        setFolders(data);
        setLoading(false);
      }catch(error){
        console.error(error);
      }
    }
    fetchFolders();
  }, [])

  console.log(folders)

  return (
    <>
      {folders?.names.map((folder) =>
        <FolderBox key={folder} folderName={folder} onSelectFolder={props.onSelectFolder}/>
      )}
    </>
  );
}
function FoldersPage(props: {onSelectFolder: (folderName: string) => void}){
  return(
    <div>
      <h1 className="description-text">Select type of file to download</h1>
      <div className="container">
        <FolderGrid onSelectFolder={props.onSelectFolder}/>
      </div>
    </div>

  );
}

export default FoldersPage;
