import React, { useState }  from 'react';
import { Code } from './remote';

// Amount of time to show immediate user feedback
let USER_FEEDBACK_TIMER = 5000;

const CodeView = () => {
  let [ code , setCode ] = useState("");
  let [ key, setKey ] = useState("");
  let [ generated , setGenerated ] = useState(false);
  let [ errorMsg, setErrorMsg] = useState("");

  let onError = (err: Error) => {
    console.error('got error from backend', err)
    setErrorMsg(err.message);
  }

  //window.remote.listContacts().then(console.log).catch(console.error)

  function handleChange (event) {
    console.log('change', event.target.value)
    setErrorMsg("");
    setCode(event.target.value)
  }

  function onClickRedeem () {
    window.remote.redeemCode(code)
      .then((message: string) => {
        console.log('got', message)
        setKey(message)
        setErrorMsg("");
      })
      .catch((err: Error) => {
        onError(err)
      })
  }

  function onClickGenerate () {
    // When a new code is generated
    // no news is good news.
    setGenerated(true);
    setErrorMsg("");
    let filename = 'fakefilename.txt'

    // Reset the state after a certain amount of time
    setTimeout(() => {
      setGenerated(false);
    }, USER_FEEDBACK_TIMER);

    window.remote.generateCode(filename)
      .then((message: string) => {
        console.log('got key', message)
        setKey(message)
        setErrorMsg("");
      })
      .catch((err: Error) => {
        setGenerated(false);
        onError(err)
      })
  }

  return (
    <div>
      <h1>MAGIC WORMHOLE</h1>
      <div className="Hello">
        <button disabled={generated} onClick={onClickGenerate}>
            Generate
        </button>

        <input type="text" onChange={handleChange}></input>
        <button onClick={onClickRedeem}>
           Redeem 
        </button>
      </div>
      <div>{errorMsg}</div>
      <div className="Code">
        {generated && "Link copied!"}
        </div>
      <div className="Key">
        {key && "Symmetric key: " + key} 
      </div>
    </div>
  );
};

export default function App() {
  return (
    <CodeView />
  );
}
