from imutils.video import VideoStream
from flask import Response
from flask import Flask
from flask import render_template
import threading
import argparse
import time
import cv2
from pathlib import Path

PORT = 3000
HOST = "0.0.0.0"

# frame that will be sent to the client
outputFrame = None

# initialise flask
app = Flask(__name__)

wCap = cv2.VideoCapture(0)
time.sleep(2.0)

@app.route("/")
def index():
    return render_template("index.html")

def generate():

    global wCap, outputFrame

    while True:

        (flag, frame) = wCap.read()

        if not flag:
            print("failed to read video stream")
            continue

        outputFrame = frame.copy()

        if outputFrame is None:
            continue

        (flag, img) = cv2.imencode(".png", outputFrame)

        if not flag:
            continue

        yield(b'--frame\r\n' b'Content-Type: image/png\r\n\r\n' + 
			bytearray(img) + b'\r\n')

@app.route("/video_feed")
def video_feed():
    return Response(generate(),
        mimetype="multipart/x-mixed-replace; boundary=frame")

app.run(host=HOST, port=PORT, debug=True, threaded=False, use_reloader=False)

# add this somewhere
# frame = wCap.read()
# outputFrame = frame.copy()