import { useEffect, useState } from "react"
import "../styles/fileContainer.css"
import "../styles/searchBar.css"

interface File{
    name: string
    size_mb: number
    extension: string
    path: string
    modified: string
}

function SearchBar(props: {updateFilter: React.Dispatch<React.SetStateAction<string>>}){
    const [searchText, setSearchText] = useState("");
    function handleSearchChange(e: {target: {value: string}}){
        setSearchText(e.target.value);
        props.updateFilter(searchText);
    }
    return(
        <div className="search-bar-container">
            <input value={searchText} onChange={handleSearchChange} className="search-bar" type="text" placeholder="Search files..."></input>
        </div>
    );
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

function FileList(props: {files: File[], folderName:string, filter: string}){
    let filteredFiles = props.files.filter((file)=> file.name.toLowerCase().includes(props.filter));
    if(filteredFiles.length == 0){
        return (
            <>
                {
                    props.files.map((file) =>
                    <FileContainer key={file.name} file={file} folder={props.folderName}/>
                )}
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
//Corregir : no queremos que llame a la api cada que se modifica el filtro, 
// solo queremos que se filtre cada que se modifica el filtro
// solo busca hasta que hayd dos segundo caracter?
function FileInfo(props: {folderName: string}){
    const [searchFilter, setSearchFilter] = useState("");
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

    return(
        <>
            <SearchBar updateFilter={setSearchFilter}/>
            <FileList files={files} filter={searchFilter} folderName={props.folderName}/>
        </>
    )
}

export default FileInfo;