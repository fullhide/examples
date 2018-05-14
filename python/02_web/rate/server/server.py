#参考 https://qiita.com/chromabox/items/a1323225bae146c80bec

import requests
from datetime import datetime

def get_time():
    return str(datetime.now())

URL = "https://www.gaitameonline.com/rateaj/getrate"

rate_data_cache = None

def get_rate():
    global rate_data_cache
    if not rate_data_cache:
        rt = requests.get(URL).json()
        rt["getTime"] = get_time()
        rate_data_cache = rt
    return rate_data_cache

def refresh_rate():
    global rate_data_cache
    rate_data_cache = None
    return get_rate()


import flask
from flask import jsonify


PORT = 5678
app = flask.Flask(__name__)

@app.route("/")
def index():
    return app.send_static_file("index.html")


@app.route("/api/rate/")
def rate():
    rt = get_rate()
    return jsonify(rt)

@app.route("/api/rate/latest")
def rate_latest():
    rt = refresh_rate()
    return jsonify(rt)


@app.route("/api/hello/")
def hello():
    result = {
        "Result": {
            "Greeting":"what's your name?"
        }
    }
    return flask.jsonify(ResultSet=result)

@app.route("/api/hello/<name>")
def hello__name(name):
    result = {
        "Result": {
            "Greeting": f"Hello {name} from flask! ."
        }
    }
    return flask.jsonify(ResultSet=result)

if __name__ == "__main__":
    app.run(
        host="0.0.0.0",
        port=PORT,
        threaded=True,
        debug=True
    )
