import React, { useEffect, useRef, useState } from 'react';

import init, { run, resize } from "decay";

function resizeJS()
{
    HEIGHT = window.innerHeight;
    WIDTH = window.innerWidth/2;
    resize(WIDTH, HEIGHT);
}

let HEIGHT = window.innerHeight;
let WIDTH = window.innerWidth/2;

interface EmptyTest {
    proteinFile: string;
    height: number;
    width: number;
}

const divStyle = {
    backgroundColor: "rgba(1, 205, 254, 0.25)",
    innerWidth: window.innerWidth,
    innerHeight: window.innerHeight,
    height: HEIGHT,
    width: WIDTH,
    alignItems: 'center',
    display: 'grid',
    justifyContent: 'center',
  };

const innerStyle = {
    color: 'blue',
    backgroundColor: "rgba(1, 205, 254, 0.25)",
    innerWidth: window.innerWidth,
    innerHeight: window.innerHeight,
    height: HEIGHT,
    width: WIDTH,
    alignItems: 'center',
    display: 'flex',
    justifyContent: 'center',
  };

function Decay(props: EmptyTest) {
    init().then(() => {});
    return (
        <div style={divStyle}>
            <div style={divStyle}>
                <button onClick={resizeJS}>Resize!</button>
                <button onClick={run}>Start!</button>
                <div className="Decay" id="decay" style={divStyle} />
            </div>
        </div>
    )
}

export default Decay;