import React, { useState } from 'react';
import Button from '@mui/joy/Button';

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

function Decay(props: EmptyTest) {
    init().then(() => {});
    const [hidden, setHidden] = useState(false);
    // setHidden(true); // do we need to initialize it?!  I thought use state did that!
    function swapHide() {
        setHidden(!hidden)
    }
    return (
        <div style={divStyle}>
            <div style={divStyle}>
                <Button onClick={run}>Start!</Button>
                <Button onClick={resizeJS} disabled={hidden}>Resize!</Button>
                <Button onClick={swapHide}>Hide</Button> {/* Something about modifying the state fucks shit up.  Just running this function, either as a declared function or the lambda, makes things bad. */}
                <div className="Decay" id="decay" style={divStyle} />
            </div>
        </div>
    )
}

export default Decay;