import React, { useEffect, useState } from 'react';
import logo from './logo.svg';
import * as decay from './decay';
import './App.css';
import { decorator } from '@babel/types';

// const decay = require("./decay");

import init, { run } from "./decay/decay.js";

function runWasm()
{
  init().then(() => {
    console.log("WASM Loaded");
    run();
  })
}

function App() {
  return (
    <div className="App">
    <header className="App-header">
      <img src={logo} className="App-logo" alt="logo" />
      <p>
        Edit <code>src/App.tsx</code> and save to reload.
      </p>
      <body id="wasm-example">
        <button onClick={runWasm}>Start!</button>
      </body>
      <a
        className="App-link"
        href="https://reactjs.org"
        target="_blank"
        rel="noopener noreferrer"
      >
        Learn React
      </a>
    </header>
  </div>
  );
}

export default App;
