module.exports = scenario => {

  const { config1, config2 } = require('../config')

  scenario('A subscriber can request a subscription', async (s, t) => {
    const {provider} = await s.players({provider: config1}, true)
  })
}
