import React, { useRef } from 'react';
import './App.css';
import  Decay  from "./decay-react/decay";
import '@fontsource/public-sans';

const AppStyle = {
  alignItems: 'center', 
  // width: width, 
  // height: height, 
  backgroundColor: "rgba(1, 205, 254, 0.25)",
  display: 'flex',
  justifyContent: 'center',
}

function App() {
  const windowSize = useRef([window.innerWidth, window.innerHeight]);
  const height = windowSize.current[1];
  const width = windowSize.current[0];
  return (
    <body
      className="DecayReact"
      style={AppStyle}
    >
      <Decay proteinFile="yay" width={width} height={height} />
    </body>
  );
}


export default App;
