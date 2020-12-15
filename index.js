const conf = require('./config.json')
const { exec } = require("child_process");

if (!process.argv[2]) {
    return console.log("No file was given.")
}

let strbuild = `curl -k -o - -v -F key='${conf.key}' -F file='@${process.argv[2]}' ${conf.remote}`

exec(strbuild, (nerror, stdout, stderr) => {
    if (nerror) { console.log(`error: ${nerror.message}`); }
    if (stderr) { console.log(`stderr: ${stderr}`); }
    console.log(`stdout: ${stdout}`);
})
