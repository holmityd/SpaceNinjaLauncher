(() => {
    "use strict";
  
    const path = require("path"),
      childprocess = require("child_process"),
      fs = require("fs/promises"),
      process = require("process");
  
    const MONGO_EXE_NAME = "mongod.exe",
      DB_FOLDER_NAME = "db",
      PORT = 27017;
  
    const currentFolderPath = process.execPath.substring(
        0,
        process.execPath.lastIndexOf("\\")
      ),
      databaseFolderPath = path.join(currentFolderPath, DB_FOLDER_NAME),
      mongoExePathInPkg = path.join(__dirname, MONGO_EXE_NAME),
      mongoExePath = path.join(databaseFolderPath, MONGO_EXE_NAME),
      executionArguments = [`--dbpath=${databaseFolderPath}`, `--port=${PORT}`];
  
    fs.access(databaseFolderPath)
      .catch(() => fs.mkdir(databaseFolderPath, { recursive: true }))
      .then(() => {
        fs.access(mongoExePath)
          .catch(() => fs.copyFile(mongoExePathInPkg, mongoExePath))
          .then(() => {
            childprocess.execFile(
              mongoExePath,
              executionArguments,
              (err, _, stderr) => {
                err && console.log(`error: ${err.message}`);
                stderr && console.log(`stderr: ${stderr}`);
              }
            );
          });
      })
      .catch((e) => console.error("Error:", e));
  })();
  