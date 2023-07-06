import { createServer } from "http";
import { Server } from "socket.io";

const express = require('express');
const app = express();

const httpServer = createServer(app);
const io = new Server(httpServer);

app.get('/', (req, res) => {
    res.send("response");
});

app.listen(3000, () => {
    console.log("listening on port 3000");
});