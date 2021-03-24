import { EventEmitter } from 'events';
import { promisified } from 'tauri/api/tauri';

export type Code = string;

export enum ActionTopic {
  // Reserved for app-level catastrophic errors
  ERROR = 'error', 
  CODE = 'code',
  GENERATE = 'generate',
  REDEEM = 'redeem'
}

export type Action = {
  topic: ActionTopic;
  data?: Object 
}

type GenerateResponse = {
  code: Code
}

export class RPC extends EventEmitter {
  constructor (onError: Function) {
    super()
    // TODO: catch errors
  }

  generateCode (): Promise<Code> {
    return new Promise((resolve, reject) => {
      promisified({ cmd: ActionTopic.GENERATE, filename: "notafile"})
        .then((params: GenerateResponse) => {
          // TODO: verify this is a valid code
          console.log("got response from backend", params)
          resolve(params.code)
        })
        .catch(reject)
    })
  }
}

