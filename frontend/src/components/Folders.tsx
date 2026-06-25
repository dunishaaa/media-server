import { useEffect, useState } from "react";
import "../styles/folder.css"
import { useNavigate } from "react-router-dom";

interface Folders{
  names: string[]
}
function FolderBox(props: { folderName: string}){
    const navigate = useNavigate();
    return (
    <div className="box-container" onClick={() => navigate(`/media/${props.folderName}`)}>
        <h2>{props.folderName.toLowerCase()}</h2>
    </div>
    );
}

function FolderGrid(){
  const [folders, setFolders] = useState<Folders>();
  const [_, setLoading] = useState(true);
  const IP = import.meta.env.VITE_IP;
  const PORT = import.meta.env.VITE_PORT;


  useEffect(() => {
    async function fetchFolders() {
      try{
        const response = await fetch(`http://${IP}:${PORT}/api/folders`);
        //const response = await fetch(`http://192.168.1.107:3000/api/folders`);

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
        <FolderBox key={folder} folderName={folder} />
      )}
    </>
  );
}
function FoldersPage(){
  return(
    <div>
      <h1 className="description-text">Select type of file to download</h1>
      <div className="container">
        <FolderGrid/>
      </div>
    </div>

  );
}

export default FoldersPage;
