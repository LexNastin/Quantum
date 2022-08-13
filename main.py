from PIL import Image, ImageTk
import tkinter as tk
from Shard import Shard
from API import API
import threading
import flask
import time
import math
import io

shard = Shard()

api = API(shard)

flask_app = flask.Flask(__name__)
flask_app.register_blueprint(api.api)
def _thread():
    flask_app.run(host="0.0.0.0")

thread = threading.Thread(target=_thread, daemon=True)
thread.start()

while(1):
    shard.update()