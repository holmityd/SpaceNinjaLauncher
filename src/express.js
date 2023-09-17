// express.js

const express = require("express");
// import express from 'express';
const app = express();
const port = 3000;

const routes = [
    {
        path: "/",
        handler: (req, res) => {
            const responseData = {
                message: "Hello World!",
                date: new Date()
            };
            res.json(responseData);
        }
    }
];

function getRoutes(app) {
    routes?.forEach(router => {
        app.get(router.path, router.handler);
    });
    return app;
}

getRoutes(app).listen(port, () => {
    console.log(`Server is running on http://localhost:${port}`);
});
