
import React from 'react'
import { render } from 'react-dom'
import { fetch } from 'node-fetch'

import sample from './sample'

// sample()


const URL = "http://localhost:5678/api/rate/"

// (async () => {
//   try {
//     const response = await fetch(url)
//     const json = await response.json()
//     data = json
//   } catch (error) {
//     data = err
//   }
// })();


class App extends React.Component {
    constructor(props) {
      super(props)
      this.state = { message: 'something' }
    }

    onChange(e) {
      this.setState({ message: e.target.value })
    }

    render() {
      return (
        <div>
          <input
            type="text"
            onChange={this.onChange.bind(this)}
          />
          <p>{this.state.message}</p>
        </div>
      )
    }
  }

render(<App />, document.getElementById('app'))
