import React, { useState }  from 'react';
import { Code } from './remote';

// Amount of time to show immediate user feedback
let USER_FEEDBACK_TIMER = 5000;

const CodeView = () => {
  let [ code , setCode ] = useState("");
  let [ disabled , setDisabled ] = useState(false);

  let onError = (err: Error) => {
    console.error('got error from backend', err)
  }

  function handleChange (event) {
    console.log('got event', event)
    console.log('set code', event.target.value)
    setCode(event.target.value)
  }

  function onClickRedeem () {
    window.remote.redeemCode(code)
      .then((message: string) => {
        console.log('got', message)
      })
      .catch((err: Error) => {
        onError(err)
      })
  }

  function onClickGenerate () {
    // When a new code is generated
    setDisabled(true);
    let filename = 'fakefilename.txt'
    window.remote.generateCode(filename)
      .then((message: Code) => {

        // Write the code to the clipboard and notify the user
        setCode(code);

        // Reset the state after a certain amount of time
        setTimeout(() => {
          setCode("");
          setDisabled(false);
        }, USER_FEEDBACK_TIMER);
      })
      .catch((err: Error) => {
        onError(err)
      })
  }

  return (
    <div>
      <h1>DANA</h1>
      <div className="Hello">
        <button disabled={disabled} onClick={onClickGenerate}>
            Generate
        </button>

        <input type="text" value={code} onChange={handleChange}></input>
        <button onClick={onClickRedeem}>
           Redeem 
        </button>
      </div>
      <div className="Code">
        {code && "Link copied!"}
      </div>
    </div>
  );
};

export default function App() {
  return (
    <CodeView />
  );
}
