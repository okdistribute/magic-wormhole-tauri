import path from 'path'

export default {
  PORT: 3163,
  ROOT_PATH: path.join(__dirname, '..'),
  IS_DEV: process.env.NODE_ENV === 'development',
  IS_PRODUCTION: process.env.NODE_ENV === 'production'
}
