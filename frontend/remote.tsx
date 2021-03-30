import TauriClient from './tauri'
import WebRTC from './webrtc'

export let RPC;

if (window.__TAURI_INVOKE_HANDLER__) {
  RPC = TauriClient;
} else {
  RPC = WebRTC; 
}

