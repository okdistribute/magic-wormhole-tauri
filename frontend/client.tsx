import { EventEmitter } from 'events';
import type { Code } from './types';

export default class DanaClient extends EventEmitter {
  constructor (onError: Function) {
    super()
    // TODO: catch errors
  }

  generateCode (filename: string): Promise<Code> {
    throw new Error('Not implemented.')
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