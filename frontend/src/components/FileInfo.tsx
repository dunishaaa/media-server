import { useEffect, useState } from "react"
import "../styles/fileContainer.css"
import "../styles/searchBar.css"
import Home from "./HomeButton"
import { useParams } from "react-router-dom"

interface File{
    name: string
    size_mb: number
    extension: string
    path: string
    modified: string
}

function SearchBar(props: {setFilter: React.Dispatch<React.SetStateAction<string>>}){
    const [searchText, setSearchText] = useState("");
    function handleSearchChange(e: {target: {value: string}}){
        const newValue = e.target.value;
        setSearchText(newValue);
        props.setFilter(newValue);
        console.log(`filter from handleSearchChange() =  ${searchText}`);
        console.log(`e.target.value from handleSearchChange() =  ${e.target.value}`);
    }
    return(
        <div className="search-bar-container">
            <input value={searchText} onChange={handleSearchChange} className="search-bar" type="text" placeholder="Search files..."></input>
        </div>
    );
}

function FileContainer(props: {folder: string, file: File}){
    let file = props.file;
    console.log("From file container -> " + props.folder)
    return(
        <div className="files-container">
            <div className="file-name">{file.name}</div>
            <div className="file-metadata">
                <div className="file-path">{file.path}</div>
                <div className="file-size">Size: {file.size_mb} MB</div>
            </div>
            <div className="button-container">
                <a className="download-button" download={file.name} href={`/download/${props.folder}/${encodeURIComponent(file.name)}`}>Download</a>
            </div>
        </div>

    )
}

function FileList(props: {files: File[], folderName:string, filter: string}){
    console.log(`filter from FileList() = ${props.filter}`);
    console.log(`folderName from FileList() = ${props.folderName}`);
    let filteredFiles = props.files.filter((file)=> file.name.toLowerCase().includes(props.filter));
    if(filteredFiles.length == 0){
        return (
            <>
            <div>Busca alguito que exista</div>
                {
                    props.files.map((file) =>
                    <FileContainer key={file.name} file={file} folder={props.folderName}/>
                    )
                }
            </>
        )
    }else{
        return(
            <>
                {
                    filteredFiles
                    .map((file) =>
                        <FileContainer key={file.name} file={file} folder={props.folderName}/>
                    )
                }
            </>
        )
    }
    
}
// solo busca hasta que hayd dos segundo caracter?
function FileInfo(){
    const {folderPath} = useParams();
    const [searchFilter, setSearchFilter] = useState("");
    const [files, setFiles] = useState<File[]>([]);

    const IP = import.meta.env.VITE_IP;
    const PORT = import.meta.env.VITE_PORT;

    console.log("From fileInfo -> " + folderPath)

    useEffect(() => {
        async function fetchFileInfo(){
            try{
                const response = await fetch(`http://${IP}:${PORT}/api/files/${folderPath}`);
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

    return(
        <>
            <Home/>
            <SearchBar setFilter={setSearchFilter}/>
            <FileList files={files} filter={searchFilter} folderName={folderPath!}/>
        </>
    )
}

export default FileInfo;
