import React, { useEffect, useRef, useState } from 'react';

import init, { run, resize } from "../decay/decay.js";

interface EmptyTest {
    proteinFile: string;
    height: number;
    width: number;
}

class Decay extends React.Component<EmptyTest>
{
    state: EmptyTest;
    constructor(props: EmptyTest) {
        super(props);
        this.state = { proteinFile: "", height: props.height, width: props.width } as EmptyTest;
        init();
    }

    runWasm()
    {
        return async () => {
            await run();
        }
    }

    resizeJS()
    {
        // resize(this.state.width, this.state.height);
        resize(100, 100);
    }

    render() {
        return (
            <div className="Decay">
              <body id="wasm-example">
                {/* <button onClick={this.runWasm}>Start!</button> */}
                <button onClick={this.resizeJS}>Resize!</button>
              </body>
            </div>
        )
    }
}

export { Decay }