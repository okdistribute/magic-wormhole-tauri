import { EventEmitter } from 'events';
import { promisified } from 'tauri/api/tauri';

export type Code = string;

export enum ActionTopic {
  // TODO: create these programmatically from cmd.rs
  GENERATE = 'generateCode',
  REDEEM = 'redeemCode',
  ADD_CONTACT = 'addContact',
  LIST_CONTACTS = 'listContacts'
}

export type Action = {
  topic: ActionTopic;
  data?: Object 
}

type Response = {
  message: string
}

export class RPC extends EventEmitter {
  constructor (onError: Function) {
    super()
    // TODO: catch errors
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

  addContact (key: string, name: string): Promise<string> {
    return new Promise((resolve, reject) => {
      promisified({ cmd: ActionTopic.ADD_CONTACT, key, name })
        .then((params: Response) => {
          console.log("got response from backend", JSON.stringify(params))
          resolve(params.message)
        })
        .catch(reject)
    })
  }

  listContacts (): Promise<any> {
    return new Promise((resolve, reject) => {
      promisified({ cmd: ActionTopic.LIST_CONTACTS })
        .then((params: Response) => {
          console.log("got response from backend", JSON.stringify(params))
          resolve(params)
        })
        .catch(reject)
    })
  }
}

