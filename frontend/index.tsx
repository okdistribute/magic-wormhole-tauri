import React from 'react';
import { render } from 'react-dom';
import App from './App';
import { RPC } from './remote';
import config from '../config';

declare global {
  interface Window { remote: RPC; }
}

function onError (err: Error) {
  console.error("Connection error")
  console.error(err)
}

window.remote = window.remote || new RPC(onError)

var el = document.createElement('div')
el.setAttribute('id', 'root')
document.body.appendChild(el)

render(
  <App />, 
  document.getElementById('root')
);
