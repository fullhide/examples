import React from "react"
import PropTypes from "prop-types"

const Input = ({ label, text, type, id, value, handleChange }) => (
  <div className="form-group">
    <label htmlFor={label}>{text}</label>
    <input className="form-control"
      type={type}
      id={id}
      value={value}
      onChange={handleChange}
      required
    />
  </div>
)

Input.propTypes = {
  label: PropTypes.string.isRequired,
  text: PropTypes.string.isRequired,
  type: PropTypes.string.isRequired,
  id: PropTypes.string.isRequired,
  value: PropTypes.string,
  handleChange: PropTypes.func.isRequired
};

export default Input


