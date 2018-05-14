require('babel-core/register'); // development.jsでES6を使えるようにする

if (process.env.NODE_ENV === 'production') {
  console.log("***** production *****")
  module.exports = require('./webpack.conf.production');
} else {

  console.log("***** developmet *****")
  module.exports = require('./webpack.conf.development');
}
