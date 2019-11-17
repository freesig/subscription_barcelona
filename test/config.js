const { Config } = require('@holochain/try-o-rama')
const dnaPath = "./dist/subscription_barcelona.dna.json"
const provider = Config.dna(dnaPath, 'provider')
const subscriber = Config.dna(dnaPath, 'subscriber')


module.exports = {
  config1: {
    instances: {
      happ: provider
    },
    bridges: []
  },
  config2: {
    instances: {
      happ: subscriber
    },
    bridges: []
  }
}
