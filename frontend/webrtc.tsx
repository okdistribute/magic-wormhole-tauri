import DanaClient from './client';
import type { Code } from './types';
import spake2  from 'spake2-wasm';

let appid = 'newdesign/petnames/text-or-file-xfer'

export default class TauriClient extends DanaClient {
  constructor (onError: Function) {
    super(onError)
  }

  generateCode (filename: string): Promise<Code> {
    let password = 'letthem-eatcake';
    let spake2State = spake2.start(appid, password);
    let outbound = spake2.msg(spake2State);
    console.log(outbound);
    return new Promise((resolve, reject) => {
        resolve('hi');
    })
  }

  redeemCode (code: string): Promise<string> {
    throw new Error('Not implemented.')
  }

  list () {
    throw new Error('Not implemented.')
  }

  connect (peer) {
    throw new Error('Not implemented.')
  }
}