#参考 https://qiita.com/chromabox/items/a1323225bae146c80bec

import requests

URL = "https://www.gaitameonline.com/rateaj/getrate"

def get_rate():
    return requests.get(URL).json()



import flask
PORT = 5678
app = flask.Flask(__name__)

@app.route("/")
def index():
    return app.send_static_file("index.html")

app.run(
    port=PORT
)
