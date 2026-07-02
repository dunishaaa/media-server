
import { useState } from "react";
import "../styles/downloads.css"
let qualities_height = ["4k", "1440", "1080", "720", "480", "360", "240", "144"];
function DownloadPage(){
  return(
    <div className="page-container">
      <h1 className="header">Descarga de youtube</h1>
        <div className="quality-container">
          <MediaType/>
        </div>
        <div className="search-bar-container">
          <input className="search-bar" type="text" placeholder="URL de video..."/>
        </div>
          <h2 className="header2">
            Calidad
          </h2>
        <div className="quality-container">
            {
                qualities_height.map((height)=>
                  <QualityCheckBox height={height}/>
                )
            }

        </div>
    </div>
  );
}

function MediaType(){
  let [video, setVideo] = useState(true);
  let [audio, setAudio] = useState(false);
  function handleSelection(who: string){
    if(who == "v"){
      setVideo(true);
      setAudio(false);
    }else if(who == "a"){
      setVideo(false);
      setAudio(true);
    }

  }
  return (
    <>
      <input className="checkbox-box" type="checkbox" name="video_checkbox" checked={video} onClick={() => handleSelection("v")}/> <div className="quality-text">Video y audio</div>
      <input className="checkbox-box" type="checkbox" name="audio_checkbox" checked={audio} onClick={() => handleSelection("a")}/> <div className="quality-text">Audio </div>
    </>
  );
}

function QualityCheckBox(props: {height: string}){
  let [checked, setCheck] = useState(false)
  return (
    <>
        <input className="checkbox-box" type="checkbox" name={`q_${props.height}`}/>
        <div className="quality-text">
            {props.height}
        </div>
    </>

  );

}

export default DownloadPage;
