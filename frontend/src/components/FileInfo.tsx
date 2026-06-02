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
    return(
        <div className="files-container">
            <div className="file-name">{file.name}</div>
            <div className="file-metadata">
                <div className="file-path">{file.path}</div>
                <div className="file-size">Size: {file.size_mb} MB</div>
            </div>
            <div className="button-container">
                <a className="download-button" href={`/download/${props.folder}/${encodeURIComponent(file.name)}`}>Download</a>
            </div>
        </div>

    )
}

function FileList(props: {files: File[], folderName:string, filter: string}){
    console.log(`filter from FileList() = ${props.filter}`);
    let filteredFiles = props.files.filter((file)=> file.name.toLowerCase().includes(props.filter));
    if(filteredFiles.length == 0){
        return (
            <>
            <div>Busca alguito que exista</div>
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
            <SearchBar setFilter={setSearchFilter}/>
            <FileList files={files} filter={searchFilter} folderName={props.folderName}/>
        </>
    )
}

export default FileInfo;