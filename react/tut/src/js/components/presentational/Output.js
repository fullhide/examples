import React from "react"
import PropTypes from "prop-types"

const Output = ({ text, id }) => (
  <div className="output-area"
    id={id}>
    {text}
  </div>
)

Output.propTypes = {
  text: PropTypes.string,
  id: PropTypes.string.isRequired
};

export default Output

