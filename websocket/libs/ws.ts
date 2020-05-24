import * as Websocket from 'ws'

const ws = new Websocket('ws://echo.websocket.org')

ws.on('open', function open() {
  console.log('open')
  ws.send('h2')
})

ws.on('message', function incoming(data) {
  console.log(data)
  ws.close()
})
