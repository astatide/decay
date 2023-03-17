import React, { useEffect, useRef, useState } from 'react';
import logo from './logo.svg';
import * as decay from './decay';
import './App.css';
import { decorator } from '@babel/types';
import { Decay } from "./decay-react/decay";

function App() {
  const windowSize = useRef([window.innerWidth, window.innerHeight]);
  const height = windowSize.current[1];
  const width = windowSize.current[0];
  return (
    <div 
    className="Decay"
    style={{width: width, height: height, backgroundColor: "rgba(1, 205, 254, 0.25)"}}
    >
    <Decay proteinFile="yay" width={width} height={width} />
    </div>
  );
}

export default App;
