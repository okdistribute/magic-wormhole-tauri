import React, { useState }  from 'react';

// Amount of time to show immediate user feedback
let USER_FEEDBACK_TIMER = 5000;

const CodeView = () => {
  let [ code , setCode ] = useState("");
  let [ key, setKey ] = useState("");
  let [ generated , setGenerated ] = useState(false);

  let onError = (err: Error) => {
    console.error('got error from backend', err)
  }

  function handleChange (event) {
    console.log(event.target.value)
    setCode(event.target.value)
  }

  function onClickRedeem () {
    window.remote.redeemCode(code)
      .then((message: string) => {
        console.log('got', message)
        setKey(message)
      })
      .catch((err: Error) => {
        onError(err)
      })
  }

  function onClickGenerate () {
    // When a new code is generated
    // no news is good news.
    setGenerated(true);
    let filename = 'fakefilename.txt'

    // Reset the state after a certain amount of time
    setTimeout(() => {
      setGenerated(false);
    }, USER_FEEDBACK_TIMER);

    window.remote.generateCode(filename)
      .then((message: string) => {
        console.log('got', message)
        setKey(message)
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
