import React, { Component } from "react"
import ReactDOM from "react-dom"
import Input from "../presentational/Input"
import Output from "../presentational/Output"
// import sleep from '../../util/sleep'

function calc(expr) {
  try {
    return eval(expr)
  } catch (e) {
    return "e"
  }
}


class FormContainer extends Component {
  constructor() {
    super();
    this.state = {
      input_expr: "",
      output_value: ""
    }
    this.handleChange = this.handleChange.bind(this)
  }

  handleChange(event) {
    const in_value = event.target.value
    this.setState({ [event.target.id]: in_value })
    const out_value = `${calc(in_value)}`
    this.setState({"output_value": out_value})
  }

  render() {
    const { seo_title, output_value } = this.state
    return (
      <form id="article-form">
        <Input
          text="Expression"
          label="expr"
          type="text"
          id="in-expr"
          value={seo_title}
          handleChange={this.handleChange}
        />

        <Output text={output_value} id="seo-title-output" />

      </form>
    )
  }
}

export default FormContainer


const wrapper = document.getElementById("create-article-form")
if (wrapper) {
  ReactDOM.render(<FormContainer />, wrapper)
}

