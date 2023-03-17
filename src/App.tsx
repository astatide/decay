import React, { useEffect, useState } from 'react';
import logo from './logo.svg';
import * as decay from './decay';
import './App.css';
import { decorator } from '@babel/types';
import { Decay } from "./decay-react/decay";

function App() {
  return (
    <div 
    className="Decay"
    style={{width: 1000, height: 1000, backgroundColor: "rgba(1, 205, 254, 0.25)"}}
    >
    <Decay />
    </div>
  );
}

export default App;
