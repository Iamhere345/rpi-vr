const cv = require('@u4/opencv4nodejs');
const fs = require('fs');
const path = require('path');
const express = require('express');
const app = express();
const server = require('http').Server(app);
const io = require('socket.io')(server);

const FPS = 30;

//fs.writeFile(path.join(__dirname, 'out.txt'), cv.toString());

//console.log(cv);


//throw Error;

app.get('/', (req, res) => {
    res.sendFile(path.join(__dirname, 'index.html'));
});


setInterval(() => {
    const screenCap = new cv.VideoCapture(0);

    console.log("E")

    // read image from the video stream
    const frame = screenCap.read();

    // encode image to send to client
    const image = cv.imencode('.png', frame).toString('base64');

    io.emit('image', image);

}, 1000 / FPS);

server.listen(3000);