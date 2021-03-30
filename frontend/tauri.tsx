import { promisified } from 'tauri/api/tauri';
import DanaClient from './client';
import type { Code } from './types';

export enum ActionTopic {
  // TODO: create these programmatically from cmd.rs
  ERROR = 'error', 
  CODE = 'code',
  GENERATE = 'generateCode',
  REDEEM = 'redeemCode',
  HEARTBEAT = 'heartbeat'
}

export type Action = {
  topic: ActionTopic;
  data?: Object 
}

type Response = {
  message: string
}

export default class TauriClient extends DanaClient {
  constructor (onError: Function) {
    super(onError)
  }

  generateCode (filename: string): Promise<Code> {
    return new Promise((resolve, reject) => {
      promisified({ cmd: ActionTopic.GENERATE, filename})
        .then((params: Response) => {
          // TODO: verify this is a valid code
          console.log("got response from backend", JSON.stringify(params))
          resolve(params.message)
        })
        .catch(reject)
    })
  }

  redeemCode (code: string): Promise<string> {
    return new Promise((resolve, reject) => {
      promisified({ cmd: ActionTopic.REDEEM, code })
        .then((params: Response) => {
          console.log("got response from backend", JSON.stringify(params))
          resolve(params.message)
        })
        .catch(reject)
    })
  }
}
