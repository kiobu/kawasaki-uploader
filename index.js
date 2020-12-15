const conf = require('./config.json')
const { exec } = require("child_process");

if (!process.argv[2]) {
    return console.log("No file was given.")
}

let strbuild = `curl -v -X POST -F key='${conf.key}' -F file='@${process.argv[2]}' ${conf.remote}`

exec(strbuild, (nerror, stdout, stderr) => {
    if (nerror) { return console.log(`error: ${nerror.message}`); }
    if (stderr) { return console.log(`stderr: ${stderr}`); }
    console.log(`stdout: ${stdout}`);
})