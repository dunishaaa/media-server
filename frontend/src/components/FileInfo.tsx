import { useEffect, useState } from "react"
import "../styles/fileContainer.css"

interface File{
    name: string
    size_mb: number
    extension: string
    path: string
    modified: string
}
function FileContainer(props: {folder: string, file: File}){

    let file = props.file;
    return(
        <div className="files-container">
            <div className="file-name">{file.name}</div>
            <div className="file-metadata">
                <div className="file-path">{file.path}</div>
                <div className="file-size">Size: {file.size_mb} MB</div>
            </div>
            <div className="button-container">
                <a href={`/download/${props.folder}/${encodeURIComponent(file.name)}`}>Download</a>
            </div>
        </div>

    )
}
function FolderList(props: {folderName:string}){
    const [files, setFiles] = useState<File[]>([]);
    useEffect(() => {
        async function fetchFileInfo(){
            try{
                const response = await fetch(`http://192.168.1.80:3000/api/files/${props.folderName}`);
                if(!response.ok){
                    throw new Error("Failed to fetch media names");
                }
                const data: File[] = await response.json();
                setFiles(data);
            }catch(error){
                console.error(error)
            }
        }
        fetchFileInfo();
    }, []);
    console.log(files);
    return (
        <>
            {files?.map((file) =>
                <FileContainer key={file.name} file={file} folder={props.folderName}/>
            )}
        </>
    )
}
function SearchBar(){
    return(
        <div>
            <input className="search-bar" type="text" placeholder="Search files..."></input>
        </div>
    );
}
function FileInfo(props: {folderName: string}){
    return(
        <>
            <SearchBar/>
            <FolderList folderName={props.folderName}/>
        </>
    )
}

export default FileInfo;