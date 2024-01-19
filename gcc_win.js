const { exec } = require('child_process');
const path = require('path');

// Get the filename from the command line arguments
const fileName = process.argv[2];
const run = process.argv[3] === "run" ? true : false;
const inputPath = path.resolve(`workspace/${fileName}.c`);
const outputPath = path.resolve(`workspace/gcc-out/${fileName}.exe`);
const asmOutputPath = path.resolve(`workspace/gcc-out/${fileName}.s`);

function handleOutput(error, _, stderr) {
    if (error) {
        console.error(`Error: ${error.message}`);
        return;
    }
    else if (stderr) {
        console.error(`GCC Error: ${stderr}`);
        return;
    }
}

const compileCmd = `gcc ${inputPath} -o ${outputPath}`;
const asmCmd = `gcc -S -O3 -fno-asynchronous-unwind-tables ${inputPath} -o ${asmOutputPath}`;

console.log(`\n*** Running ${compileCmd} ***`);

exec(compileCmd, {'shell':'powershell.exe'}, (error, stdout, stderr) => {
    handleOutput(error, stdout, stderr);
    if(error || stderr) return;

    // Output assembly
    console.log(`*** Running ${asmCmd} ***\n`);

    exec(asmCmd, {'shell':'powershell.exe'}, (error, stdout, stderr) => {
        handleOutput(error, stdout, stderr);
    });

    if(run) {
        setTimeout(()=> {
            console.log("\nRunning program...\n")
            exec(`${outputPath}`, {'shell':'powershell.exe'}, (error, stdout, stderr) => {
                handleOutput(error, stdout, stderr);
            });
        }, 1000);
    }
});